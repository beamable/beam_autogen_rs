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
pub struct AdminGetPlayerStatusRequest {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "tournamentId", skip_serializing_if = "Option::is_none")]
    pub tournament_id: Option<String>,
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "hasUnclaimedRewards", skip_serializing_if = "Option::is_none")]
    pub has_unclaimed_rewards: Option<bool>,
}

impl AdminGetPlayerStatusRequest {
    pub fn new(player_id: i64) -> AdminGetPlayerStatusRequest {
        AdminGetPlayerStatusRequest {
            player_id,
            tournament_id: None,
            content_id: None,
            has_unclaimed_rewards: None,
        }
    }
}

