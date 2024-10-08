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
pub struct FriendshipStatus {
    #[serde(rename = "playerId")]
    pub player_id: String,
    #[serde(rename = "friendId")]
    pub friend_id: String,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
}

impl FriendshipStatus {
    pub fn new(player_id: String, friend_id: String, is_blocked: bool) -> FriendshipStatus {
        FriendshipStatus {
            player_id,
            friend_id,
            is_blocked,
        }
    }
}

