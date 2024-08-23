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
pub struct PromotionScope {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "promotions")]
    pub promotions: Vec<models::Promotion>,
}

impl PromotionScope {
    pub fn new(name: String, promotions: Vec<models::Promotion>) -> PromotionScope {
        PromotionScope {
            name,
            promotions,
        }
    }
}

