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
pub struct PlayerReward {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "addItemRequests")]
    pub add_item_requests: Vec<models::ItemCreateRequest>,
    #[serde(rename = "changeCurrencies", skip_serializing_if = "Option::is_none")]
    pub change_currencies: Option<Vec<models::CurrencyChangeReward>>,
    #[serde(rename = "callWebhooks", skip_serializing_if = "Option::is_none")]
    pub call_webhooks: Option<Vec<models::WebhookReward>>,
    #[serde(rename = "addItems", skip_serializing_if = "Option::is_none")]
    pub add_items: Option<Vec<models::NewItemReward>>,
    #[serde(rename = "applyVipBonus", skip_serializing_if = "Option::is_none")]
    pub apply_vip_bonus: Option<bool>,
    #[serde(rename = "addCurrencyMap")]
    pub add_currency_map: std::collections::HashMap<String, String>,
}

impl PlayerReward {
    pub fn new(add_item_requests: Vec<models::ItemCreateRequest>, add_currency_map: std::collections::HashMap<String, String>) -> PlayerReward {
        PlayerReward {
            description: None,
            add_item_requests,
            change_currencies: None,
            call_webhooks: None,
            add_items: None,
            apply_vip_bonus: None,
            add_currency_map,
        }
    }
}

