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
pub struct EventRewardState {
    #[serde(rename = "pendingInventoryRewards")]
    pub pending_inventory_rewards: Box<models::EventInventoryPendingRewards>,
    #[serde(rename = "currencies", skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<models::EventInventoryRewardCurrency>>,
    #[serde(rename = "pendingCurrencyRewards", skip_serializing_if = "Option::is_none")]
    pub pending_currency_rewards: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "pendingItemRewards", skip_serializing_if = "Option::is_none")]
    pub pending_item_rewards: Option<Vec<models::ItemCreateRequest>>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::EventInventoryRewardItem>>,
    #[serde(rename = "min")]
    pub min: f64,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "earned")]
    pub earned: bool,
    #[serde(rename = "claimed")]
    pub claimed: bool,
    #[serde(rename = "pendingEntitlementRewards", skip_serializing_if = "Option::is_none")]
    pub pending_entitlement_rewards: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "obtain", skip_serializing_if = "Option::is_none")]
    pub obtain: Option<Vec<models::EventRewardObtain>>,
}

impl EventRewardState {
    pub fn new(pending_inventory_rewards: models::EventInventoryPendingRewards, min: f64, earned: bool, claimed: bool) -> EventRewardState {
        EventRewardState {
            pending_inventory_rewards: Box::new(pending_inventory_rewards),
            currencies: None,
            pending_currency_rewards: None,
            pending_item_rewards: None,
            items: None,
            min,
            max: None,
            earned,
            claimed,
            pending_entitlement_rewards: None,
            obtain: None,
        }
    }
}

