pub use crate::models::watcher_notifier_spec::WatcherNotifierSpec;
pub use crate::watching::WatchTypes;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatcherItemSpec {
    /// targetKind is the kind of object that should be watched.
    #[serde(rename = "targetKind", skip_serializing_if = "Option::is_none")]
    pub target_kind: Option<String>,
    /// namespaces is an optional array of namespaces this watch should apply to.
    #[serde(rename = "namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// Selector is a label query over pods that should match the replica count. Label keys and values that must match in order to be controlled by this replica set. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// Notifier is a collection of settings that relate to where notification will be sent for selected resources.
    #[serde(rename = "notifier", skip_serializing_if = "Option::is_none")]
    pub notifier: Option<WatcherNotifierSpec>,
}
