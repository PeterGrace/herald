mod models;
mod watching;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate derive_error;
extern crate handlebars;
use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use prometheus::{Encoder, GaugeVec, TextEncoder};
use tokio;
use watching::create_and_start_watchers;

lazy_static! {
    static ref HERALD_APPVER: GaugeVec = register_gauge_vec!(
        "herald_app_info",
        "static app labels that potentially only change at restart",
        &["crate_version", "git_hash"]
    )
    .unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=info");
    env_logger::init();

    let metrics_addr = ([0, 0, 0, 0], 9898).into();
    let serve_future = Server::bind(&metrics_addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_metrics))
    }));

    let appdata_gauge =
        HERALD_APPVER.with_label_values(&[env!("CARGO_PKG_VERSION"), env!("GIT_HASH")]);
    appdata_gauge.set(1.0);
    tokio::spawn(async move { serve_future.await });

    let result = create_and_start_watchers().await;
    match result {
        Ok(_) => (),
        Err(e) => info!("error: {}", e),
    }
    Ok(())
}
async fn serve_metrics(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();
    Ok(response)
}
