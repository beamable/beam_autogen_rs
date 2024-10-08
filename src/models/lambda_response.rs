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
pub struct LambdaResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl LambdaResponse {
    pub fn new(status_code: i32) -> LambdaResponse {
        LambdaResponse {
            status_code,
            body: None,
        }
    }
}

