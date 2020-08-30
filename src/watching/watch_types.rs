use crate::models::helm_release_spec::HelmRelease;
use crate::models::watcher_spec::Watcher;
use k8s_openapi::api::apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet};
use k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler;
use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::batch::v1beta1::CronJob;
use k8s_openapi::api::core::v1::{ConfigMap, Node, Pod, ResourceQuota, Secret, Service};
use k8s_openapi::api::discovery::v1beta1::Endpoint;
use k8s_openapi::api::policy::v1beta1::PodSecurityPolicy;
use strum_macros::EnumString;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumString)]
pub enum WatchTypes {
    Endpoints(Endpoint),
    Node(Node),
    Pod(Pod),
    ResourceQuota(ResourceQuota),
    DaemonSet(DaemonSet),
    StatefulSet(StatefulSet),
    ReplicaSet(ReplicaSet),
    HorizontalPodAutoscaler(HorizontalPodAutoscaler),
    Job(Job),
    CronJob(CronJob),
    PodSecurityPolicy(PodSecurityPolicy),
    ConfigMap(ConfigMap),
    Deployment(Deployment),
    Secret(Secret),
    Service(Service),
    HelmRelease(HelmRelease),
    Watcher(Watcher),
    All,
}

impl Default for WatchTypes {
    #[cfg_attr(tarpaulin, skip)]
    fn default() -> Self {
        WatchTypes::All
    }
}
