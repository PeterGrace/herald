mod watching;
mod models;

#[macro_use] extern crate log;
#[macro_use] extern crate serde;
#[macro_use] extern crate metrics_facade;
extern crate metrics_runtime;

use tokio;
use watching::create_and_start_watchers;
use metrics_runtime::{exporters::HttpExporter, observers::PrometheusBuilder, Receiver};
use std::time::{Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=info");
    env_logger::init();

    let receiver = Receiver::builder()
        .histogram(Duration::from_secs(5), Duration::from_millis(100))
        .build()
        .expect("failed to build receiver");

    let mut sink = receiver.sink();


    let controller = receiver.controller();

    let addr = "0.0.0.0:23432"
        .parse()
        .expect("failed to parse http listen address");
    let builder = PrometheusBuilder::new();
    let exporter = HttpExporter::new(controller.clone(), builder, addr);
    tokio::spawn(exporter.async_run());
    receiver.install();
    sink.update_gauge_with_labels("app_data", 1, &[
        ("version", env!("CARGO_PKG_VERSION")),
         ("hash", env!("GIT_HASH"))
         ]);
    create_and_start_watchers().await?;
    Ok(())
}
