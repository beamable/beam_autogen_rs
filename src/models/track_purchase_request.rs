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
pub struct TrackPurchaseRequest {
    #[serde(rename = "priceInLocalCurrency")]
    pub price_in_local_currency: f64,
    #[serde(rename = "skuName")]
    pub sku_name: String,
    #[serde(rename = "skuProductId")]
    pub sku_product_id: String,
    #[serde(rename = "store")]
    pub store: String,
    #[serde(rename = "obtainItems")]
    pub obtain_items: Vec<models::ItemCreateRequest>,
    #[serde(rename = "obtainCurrency")]
    pub obtain_currency: Vec<models::CurrencyChange>,
    #[serde(rename = "purchaseId")]
    pub purchase_id: String,
    #[serde(rename = "isoCurrencySymbol")]
    pub iso_currency_symbol: String,
}

impl TrackPurchaseRequest {
    pub fn new(price_in_local_currency: f64, sku_name: String, sku_product_id: String, store: String, obtain_items: Vec<models::ItemCreateRequest>, obtain_currency: Vec<models::CurrencyChange>, purchase_id: String, iso_currency_symbol: String) -> TrackPurchaseRequest {
        TrackPurchaseRequest {
            price_in_local_currency,
            sku_name,
            sku_product_id,
            store,
            obtain_items,
            obtain_currency,
            purchase_id,
            iso_currency_symbol,
        }
    }
}

