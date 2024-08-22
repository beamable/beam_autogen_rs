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
pub struct Invite {
    #[serde(rename = "playerId")]
    pub player_id: String,
    #[serde(rename = "direction")]
    pub direction: models::InvitationDirection,
}

impl Invite {
    pub fn new(player_id: String, direction: models::InvitationDirection) -> Invite {
        Invite {
            player_id,
            direction,
        }
    }
}
