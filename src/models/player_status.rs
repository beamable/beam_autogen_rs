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
pub struct PlayerStatus {
    #[serde(rename = "lastUpdateCycle")]
    pub last_update_cycle: i32,
    #[serde(rename = "tournamentId")]
    pub tournament_id: String,
    #[serde(rename = "stage")]
    pub stage: i32,
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: Vec<models::TournamentCurrencyReward>,
    #[serde(rename = "tier")]
    pub tier: i32,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "contentId")]
    pub content_id: String,
}

impl PlayerStatus {
    pub fn new(last_update_cycle: i32, tournament_id: String, stage: i32, unclaimed_rewards: Vec<models::TournamentCurrencyReward>, tier: i32, player_id: i64, content_id: String) -> PlayerStatus {
        PlayerStatus {
            last_update_cycle,
            tournament_id,
            stage,
            unclaimed_rewards,
            tier,
            group_id: None,
            player_id,
            content_id,
        }
    }
}

