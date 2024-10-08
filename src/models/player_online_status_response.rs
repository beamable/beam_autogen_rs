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
pub struct PlayerOnlineStatusResponse {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "online")]
    pub online: bool,
    #[serde(rename = "lastSeen")]
    pub last_seen: i64,
}

impl PlayerOnlineStatusResponse {
    pub fn new(player_id: i64, online: bool, last_seen: i64) -> PlayerOnlineStatusResponse {
        PlayerOnlineStatusResponse {
            player_id,
            online,
            last_seen,
        }
    }
}

