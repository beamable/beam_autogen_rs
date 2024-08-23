/*
 * Beamable API
 *
 * Autogenerated Beamable API
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReference {
    #[serde(rename = "containerHealthCheckPort", skip_serializing_if = "Option::is_none")]
    pub container_health_check_port: Option<i64>,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<models::ServiceComponent>>,
    #[serde(rename = "arm")]
    pub arm: bool,
    #[serde(rename = "checksum")]
    pub checksum: String,
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "imageId")]
    pub image_id: String,
    #[serde(rename = "imageCpuArch", skip_serializing_if = "Option::is_none")]
    pub image_cpu_arch: Option<String>,
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<models::ServiceDependencyReference>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl ServiceReference {
    pub fn new(archived: bool, service_name: String, enabled: bool, arm: bool, checksum: String, template_id: String, image_id: String) -> ServiceReference {
        ServiceReference {
            container_health_check_port: None,
            archived,
            service_name,
            enabled,
            components: None,
            arm,
            checksum,
            template_id,
            image_id,
            image_cpu_arch: None,
            dependencies: None,
            comments: None,
        }
    }
}

