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
pub struct LeaderboardAssignmentInfo {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "leaderboardId")]
    pub leaderboard_id: String,
}

impl LeaderboardAssignmentInfo {
    pub fn new(player_id: i64, leaderboard_id: String) -> LeaderboardAssignmentInfo {
        LeaderboardAssignmentInfo {
            player_id,
            leaderboard_id,
        }
    }
}

