mod watching;
mod models;

#[macro_use] extern crate log;
#[macro_use] extern crate serde;
use futures::{StreamExt, TryStreamExt, stream};
use k8s_openapi::api::apps::v1::{
    Deployment
};
use k8s_openapi::api::core::v1::{
    Secret,
    ConfigMap,
    Service
};
use kube::{
    api::{Api, ListParams, Meta},
    Client
};
use kube_runtime::{watcher};
use tokio;
use watching::WatchTypes;
use kube_runtime::utils::try_flatten_applied;
use crate::models::helm_release_spec::HelmRelease;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();
    let client = Client::try_default().await?;
    let cl_service: Api<Service> = Api::all(client.clone());
    let cl_secret: Api<Secret> = Api::all(client.clone());
    let cl_configmap: Api<ConfigMap> = Api::all(client.clone());
    let cl_deployment: Api<Deployment> = Api::all(client.clone());
    let cl_helmrelease: Api<HelmRelease> = Api::all(client.clone());
    let lp = ListParams::default()
        .allow_bookmarks();
        //.labels("kibana.k8s.elastic.co/name=eskeim");

    let service_stream = try_flatten_applied(watcher(cl_service, lp.clone()))
        .map_ok(|d| WatchTypes::Service(d))
        .boxed();
    let deployment_stream = try_flatten_applied(watcher(cl_deployment, lp.clone()))
        .map_ok(|d| WatchTypes::Deployment(d))
        .boxed();
    let secret_stream = try_flatten_applied(watcher(cl_secret, lp.clone()))
        .map_ok(|d| WatchTypes::Secret(d))
        .boxed();
    let configmap_stream = try_flatten_applied(watcher(cl_configmap, lp.clone()))
        .map_ok(|d| WatchTypes::ConfigMap(d))
        .boxed();
    let helmrelease_stream = try_flatten_applied(watcher(cl_helmrelease, lp.clone()))
        .map_ok(|hr| WatchTypes::HelmRelease(hr))
        .boxed();

    let mut combined_stream = stream::select_all(vec![
        secret_stream,
        deployment_stream,
        configmap_stream,
        service_stream,
        helmrelease_stream
    ]);
    while let Some(o) = combined_stream.try_next().await? {
        match o {
            WatchTypes::HelmRelease(hr) => info!("Got HelmRelease: {}", Meta::name(&hr)),
            WatchTypes::ConfigMap(cm) => info!("Got configmap: {}", Meta::name(&cm)),
            WatchTypes::Secret(secret) => info!("Got secret: {}", Meta::name(&secret)),
            WatchTypes::Service(service) => info!("Got Service: {}", Meta::name(&service)),
            WatchTypes::Deployment(d) => info!("Got deployment: {}", Meta::name(&d))
        }
    }
    Ok(())
}
