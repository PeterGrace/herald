pub use crate::models::watcher_notifier_spec::WatcherNotifierSpec;
pub use crate::watching::WatchTypes;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WatcherItemSpec {
    /// targetKind is the kind of object that should be watched.
    #[serde(rename = "targetKind", skip_serializing_if = "Option::is_none")]
    pub target_kind: Option<String>,
    /// namespaces is an optional array of namespaces this watch should apply to.
    #[serde(rename = "namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// Selector is a label query over pods that should match the replica count. Label keys and values that must match in order to be controlled by this replica set. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "schema_label_selector")]
    pub selector: Option<LabelSelector>,
    /// Notifier is a collection of settings that relate to where notification will be sent for selected resources.
    #[serde(rename = "notifier", skip_serializing_if = "Option::is_none")]
    pub notifier: Option<WatcherNotifierSpec>,
}

fn schema_label_selector(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    serde_json::from_value(serde_json::json!(
{
  "description": "a map of labels to search for",
  "properties": {
    "matchExpressions": {
      "description": "a list of expressions to match",
      "items": {
        "properties": {
          "key": {
            "description": "name of key",
            "type": "string"
          },
          "operator": {
            "description": "operator",
            "type": "string"
          },
          "values": {
            "items": {
              "type": "string"
            },
            "type": "array"
          }
        },
        "type": "object"
      },
      "type": "array"
    },
    "matchLabels": {
      "additionalProperties": {
        "type": "string"
      },
      "description": "a list of labels to match",
      "type": "object"
    }
  },
  "type": "object"
}

    ))
        .unwrap()
}