use kube::CustomResource;
use serde::{Deserialize, Serialize};
use crate::models::watcher_item_spec::WatcherItemSpec;

#[derive(CustomResource, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[kube(group = "vsix.me", kind = "Watcher", derive = "PartialEq", version = "v1")]
pub struct WatcherSpec {
    /// watchers is an array of watcher items.
    #[serde(rename="watchers", skip_serializing_if = "Option::is_none")]
    watchers: Option<Vec<WatcherItemSpec>>
}