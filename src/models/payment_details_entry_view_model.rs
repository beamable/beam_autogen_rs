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
pub struct PaymentDetailsEntryViewModel {
    #[serde(rename = "reference")]
    pub reference: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "quantity")]
    pub quantity: i32,
    #[serde(rename = "sku")]
    pub sku: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "subcategory", skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    #[serde(rename = "gameplace")]
    pub gameplace: String,
    #[serde(rename = "localPrice", skip_serializing_if = "Option::is_none")]
    pub local_price: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "localCurrency", skip_serializing_if = "Option::is_none")]
    pub local_currency: Option<String>,
    #[serde(rename = "providerProductId")]
    pub provider_product_id: String,
}

impl PaymentDetailsEntryViewModel {
    pub fn new(reference: String, name: String, quantity: i32, sku: String, price: i32, gameplace: String, provider_product_id: String) -> PaymentDetailsEntryViewModel {
        PaymentDetailsEntryViewModel {
            reference,
            name,
            quantity,
            sku,
            price,
            subcategory: None,
            gameplace,
            local_price: None,
            category: None,
            local_currency: None,
            provider_product_id,
        }
    }
}

