/*
 * helmreleases.helm.fluxcd.io
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 *
 * Generated by: https://openapi-generator.tech
 */

use kube::CustomResource;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[kube(group = "helm.fluxcd.io", kind = "HelmRelease", derive = "PartialEq", shortname = "hr", version = "v1")]
pub struct HelmReleaseSpec {
    #[serde(rename = "chart")]
    pub chart: crate::models::HelmReleaseChartTargetSpec,
    /// If supplied will force Helm upgrade through delete/recreate
    #[serde(rename = "forceUpgrade", skip_serializing_if = "Option::is_none")]
    pub force_upgrade: Option<bool>,
    /// The Helm version this release targets. If not supplied, it will default to v2.
    #[serde(rename = "helmVersion", skip_serializing_if = "Option::is_none")]
    pub helm_version: Option<String>,
    /// The maximum number of release revisions to keep, defaults to 10
    #[serde(rename = "maxHistory", skip_serializing_if = "Option::is_none")]
    pub max_history: Option<i32>,
    /// The Helm release name. If not supplied, it will be generated by affixing the namespace to the resource name.
    #[serde(rename = "releaseName", skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    /// If supplied will reset values on helm upgrade
    #[serde(rename = "resetValues", skip_serializing_if = "Option::is_none")]
    pub reset_values: Option<bool>,
    #[serde(rename = "rollback", skip_serializing_if = "Option::is_none")]
    pub rollback: Option<crate::models::HelmReleaseSpecRollback>,
    /// If set, will skip CRD installation for Helm v3
    #[serde(rename = "skipCRDs", skip_serializing_if = "Option::is_none")]
    pub skip_cr_ds: Option<bool>,
    /// The Helm release namespace. If not supplied, the namespace will be the same as the resource namespace.
    #[serde(rename = "targetNamespace", skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,
    /// Helm install or upgrade timeout in seconds
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Deprecated! Use valuesFrom.secretKeyRef instead
    #[serde(rename = "valueFileSecrets", skip_serializing_if = "Option::is_none")]
    pub value_file_secrets: Option<Vec<crate::models::HelmReleaseSpecValueFileSecrets>>,
    /// content of values.yaml
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
    // #[serde(rename = "valuesFrom", skip_serializing_if = "Option::is_none")]
    // pub values_from: Option<Vec<crate::models::OneOfAnyTypeAnyTypeAnyTypeAnyType>>,
    /// Wait for an upgrade to complete before marking release as successful
    #[serde(rename = "wait", skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
}

impl HelmReleaseSpec {
    pub fn new(chart: crate::models::HelmReleaseChartTargetSpec) -> HelmReleaseSpec {
        HelmReleaseSpec {
            chart,
            force_upgrade: None,
            helm_version: None,
            max_history: None,
            release_name: None,
            reset_values: None,
            rollback: None,
            skip_cr_ds: None,
            target_namespace: None,
            timeout: None,
            value_file_secrets: None,
            values: None,
            //values_from: None,
            wait: None,
        }
    }
}


