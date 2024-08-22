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
pub struct Sku {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "realPrice")]
    pub real_price: i32,
    #[serde(rename = "productIds")]
    pub product_ids: std::collections::HashMap<String, String>,
}

impl Sku {
    pub fn new(name: String, description: String, real_price: i32, product_ids: std::collections::HashMap<String, String>) -> Sku {
        Sku {
            name,
            description,
            real_price,
            product_ids,
        }
    }
}

