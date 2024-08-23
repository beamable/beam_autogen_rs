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
pub struct MailRewards {
    #[serde(rename = "currencies")]
    pub currencies: Vec<models::CurrencyChange>,
    #[serde(rename = "items")]
    pub items: Vec<models::ItemCreateRequest>,
    #[serde(rename = "applyVipBonus", skip_serializing_if = "Option::is_none")]
    pub apply_vip_bonus: Option<bool>,
}

impl MailRewards {
    pub fn new(currencies: Vec<models::CurrencyChange>, items: Vec<models::ItemCreateRequest>) -> MailRewards {
        MailRewards {
            currencies,
            items,
            apply_vip_bonus: None,
        }
    }
}

