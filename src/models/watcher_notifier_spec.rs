#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatcherNotifierSpec {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "formatTemplate", skip_serializing_if = "Option::is_none")]
    pub format_template_string: Option<String>,
}
