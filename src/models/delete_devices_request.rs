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
pub struct DeleteDevicesRequest {
    #[serde(rename = "deviceIds", skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
}

impl DeleteDevicesRequest {
    pub fn new() -> DeleteDevicesRequest {
        DeleteDevicesRequest {
            device_ids: None,
        }
    }
}

