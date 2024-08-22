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
pub struct CompletePurchaseRequest {
    #[serde(rename = "txid")]
    pub txid: i64,
    #[serde(rename = "receipt")]
    pub receipt: String,
    #[serde(rename = "priceInLocalCurrency")]
    pub price_in_local_currency: String,
    #[serde(rename = "isoCurrencySymbol")]
    pub iso_currency_symbol: String,
}

impl CompletePurchaseRequest {
    pub fn new(txid: i64, receipt: String, price_in_local_currency: String, iso_currency_symbol: String) -> CompletePurchaseRequest {
        CompletePurchaseRequest {
            txid,
            receipt,
            price_in_local_currency,
            iso_currency_symbol,
        }
    }
}

