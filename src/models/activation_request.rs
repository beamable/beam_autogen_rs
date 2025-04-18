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
pub struct ActivationRequest {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "cid")]
    pub cid: i64,
}

impl ActivationRequest {
    pub fn new(token: String, cid: i64) -> ActivationRequest {
        ActivationRequest {
            token,
            cid,
        }
    }
}

