pub struct HelmReleaseChartRepoSpec {
    name: str,
    repository: str,
    version: str,
    chartPullSecret: HelmReleaseChartPullSecretSpec
}

impl HelmReleaseChartRepoSpec {
    pub fn new() -> HelmReleaseChartRepoSpec {
        HelmReleaseChartRepoSpec {
            name: None,
            repository: None,
            version: None,
            chartPullSecret: HelmReleaseChartPullSecretSpec::new()
        }
    }
}

pub struct HelmReleaseChartPullSecretSpec {
    name: str
}
impl HelmReleaseChartPullSecretSpec {
    pub fn new() -> HelmReleaseChartPullSecretSpec {
        HelmReleaseChartPullSecretSpec {
            name: None
        }
    }
}


