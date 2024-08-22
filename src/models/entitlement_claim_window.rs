/*
 * Auth Actor
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntitlementClaimWindow {
    #[serde(rename = "open")]
    pub open: i64,
    #[serde(rename = "close")]
    pub close: i64,
}

impl EntitlementClaimWindow {
    pub fn new(open: i64, close: i64) -> EntitlementClaimWindow {
        EntitlementClaimWindow {
            open,
            close,
        }
    }
}

