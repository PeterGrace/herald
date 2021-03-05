use schemars::JsonSchema;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct HelmReleaseChartPullSecretSpec {
    name: String,
}
