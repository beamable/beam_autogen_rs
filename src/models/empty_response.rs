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
pub struct EmptyResponse {
    #[serde(rename = "result")]
    pub result: String,
}

impl EmptyResponse {
    pub fn new(result: String) -> EmptyResponse {
        EmptyResponse {
            result,
        }
    }
}

