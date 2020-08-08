use k8s_openapi::api::apps::v1::{
    Deployment
};
use k8s_openapi::api::core::v1::{
    Secret,
    ConfigMap,
    Service
};
use crate::models::helm_release_spec::HelmRelease;
use crate::models::watcher_spec::Watcher;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WatchTypes {
    ConfigMap(ConfigMap),
    Deployment(Deployment),
    Secret(Secret),
    Service(Service),
    HelmRelease(HelmRelease),
    Watcher(Watcher),
    All
}

impl Default for WatchTypes {
    fn default() -> Self { WatchTypes::All }
}
