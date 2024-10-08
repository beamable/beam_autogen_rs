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
pub struct EventRewardContent {
    #[serde(rename = "currencies", skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<models::EventInventoryRewardCurrency>>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::EventInventoryRewardItem>>,
    #[serde(rename = "min")]
    pub min: f64,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "obtain", skip_serializing_if = "Option::is_none")]
    pub obtain: Option<Vec<models::EventRewardObtain>>,
}

impl EventRewardContent {
    pub fn new(min: f64) -> EventRewardContent {
        EventRewardContent {
            currencies: None,
            items: None,
            min,
            max: None,
            obtain: None,
        }
    }
}

