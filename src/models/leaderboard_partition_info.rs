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
pub struct LeaderboardPartitionInfo {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "leaderboardId")]
    pub leaderboard_id: String,
    #[serde(rename = "isEmpty")]
    pub is_empty: bool,
    #[serde(rename = "partition", skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
}

impl LeaderboardPartitionInfo {
    pub fn new(player_id: i64, leaderboard_id: String, is_empty: bool) -> LeaderboardPartitionInfo {
        LeaderboardPartitionInfo {
            player_id,
            leaderboard_id,
            is_empty,
            partition: None,
        }
    }
}

