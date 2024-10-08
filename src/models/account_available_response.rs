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
pub struct AccountAvailableResponse {
    #[serde(rename = "available")]
    pub available: bool,
}

impl AccountAvailableResponse {
    pub fn new(available: bool) -> AccountAvailableResponse {
        AccountAvailableResponse {
            available,
        }
    }
}

