use thiserror::Error;
use k8s_openapi::api::apps::v1::{
    Deployment
};
use k8s_openapi::api::core::v1::{
    Secret,
    ConfigMap,
    Service
};
use futures::{StreamExt, TryStreamExt, stream};
use tokio::select;
use kube::{
    api::{Api, ListParams, Meta},
    Client
};
use kube_runtime::{watcher};
use kube_runtime::utils::try_flatten_applied;
use crate::models::helm_release_spec::HelmRelease;
use crate::models::watcher_spec::Watcher;
pub use crate::watching::watch_types::WatchTypes;

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("Unknown error when watching {0}: {1}")]
    Unknown(String, String)
}


pub async fn create_and_start_watchers() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let cl_service: Api<Service> = Api::all(client.clone());
    let cl_secret: Api<Secret> = Api::all(client.clone());
    let cl_configmap: Api<ConfigMap> = Api::all(client.clone());
    let cl_deployment: Api<Deployment> = Api::all(client.clone());
    let cl_helmrelease: Api<HelmRelease> = Api::all(client.clone());
    let cl_watchers: Api<Watcher> = Api::all(client.clone());
    let lp = ListParams::default()
        .allow_bookmarks();
    //.labels("kibana.k8s.elastic.co/name=eskeim");

    let service_stream = try_flatten_applied(watcher(cl_service, lp.clone()))
        .map_ok(|d| WatchTypes::Service(d))
        .map_err(|e| WatchError::Unknown(String::from("Service"),e.to_string()))
        .boxed();
    let deployment_stream = try_flatten_applied(watcher(cl_deployment, lp.clone()))
        .map_ok(|d| WatchTypes::Deployment(d))
        .map_err(|e| WatchError::Unknown(String::from("Deployment"), e.to_string()))
        .boxed();
    let secret_stream = try_flatten_applied(watcher(cl_secret, lp.clone()))
        .map_ok(|d| WatchTypes::Secret(d))
        .map_err(|e| WatchError::Unknown(String::from("Secret"),e.to_string()))
        .boxed();
    let configmap_stream = try_flatten_applied(watcher(cl_configmap, lp.clone()))
        .map_ok(|d| WatchTypes::ConfigMap(d))
        .map_err(|e| WatchError::Unknown(String::from("ConfigMap"),e.to_string()))
        .boxed();
    let helmrelease_stream = try_flatten_applied(watcher(cl_helmrelease, lp.clone()))
        .map_ok(|hr| WatchTypes::HelmRelease(hr))
        .map_err(|e| WatchError::Unknown(String::from("HelmRelease"),e.to_string()))
        .boxed();
    let watcher_stream = try_flatten_applied(watcher(cl_watchers, lp.clone()))
        .map_ok(|watched| WatchTypes::Watcher(watched))
        .map_err(|e| WatchError::Unknown(String::from("Watcher"),e.to_string()))
        .boxed();

    let mut combined_stream = stream::select_all(vec![
        secret_stream,
        deployment_stream,
        configmap_stream,
        service_stream,
        helmrelease_stream,
        watcher_stream
    ]);
    loop {
        select! {
         _ = secret_stream.poll => {info!("found secret")},
        }
    }
    while let o = combined_stream.try_next().await {
        if o.is_err() {
            debug!("{}", o.unwrap_err());
        } else {
            match o.unwrap() {
                Some(WatchTypes::Watcher(w)) => info!("Detected watcher update for: {}", Meta::name(&w)),
                Some(WatchTypes::HelmRelease(hr)) => info!("Got HelmRelease: {}", Meta::name(&hr)),
                Some(WatchTypes::ConfigMap(cm)) => info!("Got configmap: {}", Meta::name(&cm)),
                Some(WatchTypes::Secret(secret)) => info!("Got secret: {}", Meta::name(&secret)),
                Some(WatchTypes::Service(service)) => info!("Got Service: {}", Meta::name(&service)),
                Some(WatchTypes::Deployment(d)) => info!("Got deployment: {}", Meta::name(&d)),
                Some(_) => info!("Something otherwise not aware of occurred"),
                None => info!("Error on reading")
            }
        }
    }
    Ok(())


}