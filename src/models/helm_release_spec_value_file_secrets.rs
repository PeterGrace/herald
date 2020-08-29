#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelmReleaseSpecValueFileSecrets {
    /// Name of the secret, must be in the same namespace as the HelmRelease
    #[serde(rename = "name")]
    pub name: String,
}
