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
pub struct DeleteAnnouncementRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
}

impl DeleteAnnouncementRequest {
    pub fn new(symbol: String) -> DeleteAnnouncementRequest {
        DeleteAnnouncementRequest {
            symbol,
        }
    }
}

