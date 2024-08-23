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
pub struct ResultResponse {
    #[serde(rename = "result")]
    pub result: bool,
}

impl ResultResponse {
    pub fn new(result: bool) -> ResultResponse {
        ResultResponse {
            result,
        }
    }
}

