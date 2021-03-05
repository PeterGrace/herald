use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct HelmReleaseSpecRollback {
    /// If set, will prevent hooks from running during rollback
    #[serde(rename = "disableHooks", skip_serializing_if = "Option::is_none")]
    pub disable_hooks: Option<bool>,
    /// If set, will perform rollbacks for this release on upgrade failures
    #[serde(rename = "enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// If set, will force resource update through delete/recreate if needed
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// The maximum amount of retries that should be attempted for a rolled back release if retries are enabled, defaults to 5, 0 equals infinite
    #[serde(rename = "maxRetries", skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    #[serde(rename = "recreate", skip_serializing_if = "Option::is_none")]
    pub recreate: Option<bool>,
    /// If set, the upgrade of a rolled back release will be retried until the maximum amount of retries is reached
    #[serde(rename = "retry", skip_serializing_if = "Option::is_none")]
    pub retry: Option<bool>,
    /// Time in seconds to wait for any individual Kubernetes operation, defaults to 300 seconds
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// If set, will wait until the minimum number of Pods of a Deployment are in a ready state before marking the release as successful
    #[serde(rename = "wait", skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
}
