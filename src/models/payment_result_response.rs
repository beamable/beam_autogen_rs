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
pub struct PaymentResultResponse {
    #[serde(rename = "result")]
    pub result: String,
}

impl PaymentResultResponse {
    pub fn new(result: String) -> PaymentResultResponse {
        PaymentResultResponse {
            result,
        }
    }
}

