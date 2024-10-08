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
pub struct AliasAvailableResponse {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "available")]
    pub available: bool,
    #[serde(rename = "cid")]
    pub cid: i64,
}

impl AliasAvailableResponse {
    pub fn new(alias: String, available: bool, cid: i64) -> AliasAvailableResponse {
        AliasAvailableResponse {
            alias,
            available,
            cid,
        }
    }
}

