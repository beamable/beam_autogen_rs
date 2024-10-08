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
pub struct ServiceLimits {
    #[serde(rename = "beamo", skip_serializing_if = "Option::is_none")]
    pub beamo: Option<Box<models::BeamoLimits>>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<models::ContentLimits>>,
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Box<models::GatewayLimits>>,
}

impl ServiceLimits {
    pub fn new() -> ServiceLimits {
        ServiceLimits {
            beamo: None,
            content: None,
            gateway: None,
        }
    }
}

