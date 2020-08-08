#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelmReleaseChartPullSecretSpec {
    name: String
}
impl HelmReleaseChartPullSecretSpec {
    pub fn new() -> HelmReleaseChartPullSecretSpec {
        HelmReleaseChartPullSecretSpec {
            name: String::from("")
        }
    }
}


