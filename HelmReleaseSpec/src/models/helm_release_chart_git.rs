
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelmReleaseChartGitSpec {
    git: str,
    path: str,
    ref: str,
    skipDefUpdate: boolean
}
impl HelmReleaseChartGitSpec {
    pub fn new() -> HelmReleaseChartGitSpec {
        HelmReleaseChartGitSpec {
            git: None,
            path: None,
            ref: None,
            skipDepUpdate: false
        }
    }
}


