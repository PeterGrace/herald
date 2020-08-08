use k8s_openapi::api::apps::v1::{
    Deployment
};
use k8s_openapi::api::core::v1::{
    Secret,
    ConfigMap,
    Service
};
use futures::future::SelectAll;
use crate::helmreleasespec::models::HelmReleaseSpec;

pub(crate) enum WatchTypes {
    ConfigMap(ConfigMap),
    Deployment(Deployment),
    Secret(Secret),
    Service(Service),
    HelmRelease(HelmReleaseSpec),
}

fn setup_combined_watch_stream() -> anyhow::Result<()> {
Ok(())
}