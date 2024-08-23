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
pub struct RegisterReq {
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "token")]
    pub token: String,
}

impl RegisterReq {
    pub fn new(provider: String, token: String) -> RegisterReq {
        RegisterReq {
            provider,
            token,
        }
    }
}

