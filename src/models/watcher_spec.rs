pub use crate::models::watcher_item_spec::WatcherItemSpec;
use kube::CustomResource;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(CustomResource, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[kube(
    group = "vsix.me",
    kind = "Watcher",
    derive = "PartialEq",
    derive = "Default",
    version = "v1"
)]
pub struct WatcherSpec {
    /// watchers is an array of watcher items.
    #[serde(rename = "watchers", skip_serializing_if = "Option::is_none")]
    pub watchers: Option<Vec<WatcherItemSpec>>,
}
