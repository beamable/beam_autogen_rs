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
pub struct EntitlementGenerator {
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "claimWindow", skip_serializing_if = "Option::is_none")]
    pub claim_window: Option<Box<models::EntitlementClaimWindow>>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "specialization", skip_serializing_if = "Option::is_none")]
    pub specialization: Option<String>,
    #[serde(rename = "action")]
    pub action: String,
}

impl EntitlementGenerator {
    pub fn new(symbol: String, action: String) -> EntitlementGenerator {
        EntitlementGenerator {
            quantity: None,
            claim_window: None,
            params: None,
            symbol,
            specialization: None,
            action,
        }
    }
}

