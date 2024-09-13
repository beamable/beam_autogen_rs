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
pub struct CurrencyView {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "properties")]
    pub properties: Vec<models::CurrencyProperty>,
    #[serde(rename = "proxy", skip_serializing_if = "Option::is_none")]
    pub proxy: Option<Box<models::InventoryObjectFederationInfo>>,
}

impl CurrencyView {
    pub fn new(id: String, amount: i64, properties: Vec<models::CurrencyProperty>) -> CurrencyView {
        CurrencyView {
            id,
            amount,
            properties,
            proxy: None,
        }
    }
}

