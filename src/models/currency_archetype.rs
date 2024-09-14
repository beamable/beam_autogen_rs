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
pub struct CurrencyArchetype {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<Box<models::InventoryBasicFederationInfo>>,
    #[serde(rename = "clientPermission", skip_serializing_if = "Option::is_none")]
    pub client_permission: Option<Box<models::ClientPermission>>,
    #[serde(rename = "startingAmount", skip_serializing_if = "Option::is_none")]
    pub starting_amount: Option<i64>,
}

impl CurrencyArchetype {
    pub fn new(symbol: String) -> CurrencyArchetype {
        CurrencyArchetype {
            symbol,
            external: None,
            client_permission: None,
            starting_amount: None,
        }
    }
}

