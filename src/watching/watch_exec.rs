use thiserror::Error;
use k8s_openapi::api::apps::v1::{
    Deployment
};
use k8s_openapi::api::core::v1::{
    Secret,
    ConfigMap,
    Service
};
use futures::{StreamExt, TryStreamExt, stream, Stream};
use strum_macros::EnumString;
use std::str::FromStr;
use kube::{api::{Api, ListParams, Meta}, Client};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use k8s_openapi::Resource;
use kube_runtime::{watcher};
use kube_runtime::utils::try_flatten_applied;
use crate::models::helm_release_spec::HelmRelease;
use crate::models::watcher_spec::{Watcher, WatcherSpec, WatcherItemSpec};
pub use crate::watching::watch_types::WatchTypes;
use std::collections::BTreeMap;
use std::time::{Instant};
use prometheus::{HistogramVec, TextEncoder};

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("Unknown error when watching {0}: {1}")]
    Unknown(String, String)
}

lazy_static! {
    static ref HERALD_FN_HISTOGRAM: HistogramVec = register_histogram_vec!(
    "herald_fn_duration_seconds",
    "The number of seconds it took to process a watcher object",
    &["function"]
    ).unwrap();
}





pub async fn create_and_start_watchers() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let lp = ListParams::default()
        .allow_bookmarks();

    let mut stream_vec = Vec::new();

    // Node(Node),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::Node(obj))
        .map_err(|e| WatchError::Unknown(String::from("Node"),e.to_string()))
        .boxed());
    //Pod(Pod),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::Pod(obj))
        .map_err(|e| WatchError::Unknown(String::from("Pod"),e.to_string()))
        .boxed());

    //ResourceQuota(ResourceQuota),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::ResourceQuota(obj))
        .map_err(|e| WatchError::Unknown(String::from("ResourceQuota"),e.to_string()))
        .boxed());

    //DaemonSet(DaemonSet),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::DaemonSet(obj))
        .map_err(|e| WatchError::Unknown(String::from("DaemonSet"),e.to_string()))
        .boxed());
    //StatefulSet(StatefulSet),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::StatefulSet(obj))
        .map_err(|e| WatchError::Unknown(String::from("StatefulSet"),e.to_string()))
        .boxed());
    //ReplicaSet(ReplicaSet),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::ReplicaSet(obj))
        .map_err(|e| WatchError::Unknown(String::from("ReplicaSet"),e.to_string()))
        .boxed());

    //HorizontalPodAutoscaler(HorizontalPodAutoscaler),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::HorizontalPodAutoscaler(obj))
        .map_err(|e| WatchError::Unknown(String::from("HorizontalPodAutoscaler"),e.to_string()))
        .boxed());

    //Job(Job),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::Job(obj))
        .map_err(|e| WatchError::Unknown(String::from("Job"),e.to_string()))
        .boxed());
    //CronJob(CronJob),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::CronJob(obj))
        .map_err(|e| WatchError::Unknown(String::from("CronJob"),e.to_string()))
        .boxed());

    //PodSecurityPolicy(PodSecurityPolicy),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|obj| WatchTypes::PodSecurityPolicy(obj))
        .map_err(|e| WatchError::Unknown(String::from("PodSecurityPolicy"),e.to_string()))
        .boxed());

    //ConfigMap(ConfigMap),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|d| WatchTypes::ConfigMap(d))
        .map_err(|e| WatchError::Unknown(String::from("ConfigMap"),e.to_string()))
        .boxed());
    //Deployment(Deployment),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|d| WatchTypes::Deployment(d))
        .map_err(|e| WatchError::Unknown(String::from("Deployment"), e.to_string()))
        .boxed());
    //Secret(Secret),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|d| WatchTypes::Secret(d))
        .map_err(|e| WatchError::Unknown(String::from("Secret"),e.to_string()))
        .boxed());
    //Service(Service),
    stream_vec.push( try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|d| WatchTypes::Service(d))
        .map_err(|e| WatchError::Unknown(String::from("Service"),e.to_string()))
        .boxed());
    //HelmRelease(HelmRelease),
    stream_vec.push(try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|hr| WatchTypes::HelmRelease(hr))
        .map_err(|e| WatchError::Unknown(String::from("HelmRelease"),e.to_string()))
        .boxed());
    //Watcher(Watcher),
    stream_vec.push(try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
        .map_ok(|watched| WatchTypes::Watcher(watched))
        .map_err(|e| WatchError::Unknown(String::from("Watcher"),e.to_string()))
        .boxed());




    let mut combined_stream = stream::select_all(stream_vec);

    while let o = combined_stream.try_next().await {
        if o.is_err() {
            debug!("{}", o.unwrap_err());
        } else {
            match o.unwrap() {
                Some(WatchTypes::Watcher(w)) => process_watcher(w),
                Some(_) => (),
                None => info!("Error on reading")
            }
        }
    }
    Ok(())
}

fn process_watcher(w: crate::models::watcher_spec::Watcher) -> ()
{
    let timer = HERALD_FN_HISTOGRAM.with_label_values(&["process_watcher"]).start_timer();
    info!("watcher: {}", Meta::name(&w));
    for w_ in w.spec.watchers
    {
        info!("watch kind: {:#?}", w_);
    }
    timer.observe_duration();
}