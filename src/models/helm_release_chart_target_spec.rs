
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelmReleaseChartTargetSpec {
    #[serde(rename = "git", skip_serializing_if = "Option::is_none")]
    pub git: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub git_ref: Option<String>,
    #[serde(rename = "skipDefUpdate", skip_serializing_if = "Option::is_none")]
    pub skip_def_update: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    repository: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(rename = "chartPullSecret", skip_serializing_if = "Option::is_none")]
    chart_pull_secret: Option<crate::models::HelmReleaseChartPullSecretSpec>
}

