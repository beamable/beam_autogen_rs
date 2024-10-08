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
pub struct DisbandRequest {
    #[serde(rename = "subGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<i64>,
}

impl DisbandRequest {
    pub fn new() -> DisbandRequest {
        DisbandRequest {
            sub_group: None,
        }
    }
}

