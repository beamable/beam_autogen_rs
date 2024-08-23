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
pub struct ChampionScore {
    #[serde(rename = "endTimeMs")]
    pub end_time_ms: i64,
    #[serde(rename = "startTimeMs")]
    pub start_time_ms: i64,
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "cycle")]
    pub cycle: i32,
    #[serde(rename = "playerId")]
    pub player_id: i64,
}

impl ChampionScore {
    pub fn new(end_time_ms: i64, start_time_ms: i64, score: f64, cycle: i32, player_id: i64) -> ChampionScore {
        ChampionScore {
            end_time_ms,
            start_time_ms,
            score,
            cycle,
            player_id,
        }
    }
}

