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
pub struct TournamentCurrencyReward {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "amount")]
    pub amount: i64,
}

impl TournamentCurrencyReward {
    pub fn new(symbol: String, amount: i64) -> TournamentCurrencyReward {
        TournamentCurrencyReward {
            symbol,
            amount,
        }
    }
}

