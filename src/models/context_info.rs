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
pub struct ContextInfo {
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "device")]
    pub device: String,
}

impl ContextInfo {
    pub fn new(platform: String, device: String) -> ContextInfo {
        ContextInfo {
            platform,
            device,
        }
    }
}

