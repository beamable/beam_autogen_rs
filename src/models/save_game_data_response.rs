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
pub struct SaveGameDataResponse {
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "sid")]
    pub sid: i64,
    #[serde(rename = "version")]
    pub version: i64,
}

impl SaveGameDataResponse {
    pub fn new(result: String, sid: i64, version: i64) -> SaveGameDataResponse {
        SaveGameDataResponse {
            result,
            sid,
            version,
        }
    }
}

