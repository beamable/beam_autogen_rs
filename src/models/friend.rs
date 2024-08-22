/*
 * Auth Actor
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Friend {
    #[serde(rename = "playerId")]
    pub player_id: String,
    #[serde(rename = "source")]
    pub source: models::FriendSource,
}

impl Friend {
    pub fn new(player_id: String, source: models::FriendSource) -> Friend {
        Friend {
            player_id,
            source,
        }
    }
}
