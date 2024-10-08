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
pub struct EntitlementRequirement {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "constraint")]
    pub constraint: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "specialization", skip_serializing_if = "Option::is_none")]
    pub specialization: Option<String>,
}

impl EntitlementRequirement {
    pub fn new(symbol: String, constraint: String, state: String) -> EntitlementRequirement {
        EntitlementRequirement {
            symbol,
            constraint,
            state,
            specialization: None,
        }
    }
}

