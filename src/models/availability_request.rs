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
pub struct AvailabilityRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub r#type: models::GroupType,
    #[serde(rename = "subGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<bool>,
}

impl AvailabilityRequest {
    pub fn new(r#type: models::GroupType) -> AvailabilityRequest {
        AvailabilityRequest {
            name: None,
            tag: None,
            r#type,
            sub_group: None,
        }
    }
}

