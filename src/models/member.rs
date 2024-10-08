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
pub struct Member {
    #[serde(rename = "canDemote", skip_serializing_if = "Option::is_none")]
    pub can_demote: Option<bool>,
    #[serde(rename = "canKick", skip_serializing_if = "Option::is_none")]
    pub can_kick: Option<bool>,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "gamerTag")]
    pub gamer_tag: i64,
    #[serde(rename = "canPromote", skip_serializing_if = "Option::is_none")]
    pub can_promote: Option<bool>,
    #[serde(rename = "scores", skip_serializing_if = "Option::is_none")]
    pub scores: Option<Vec<models::GroupScoreBinding>>,
}

impl Member {
    pub fn new(role: String, gamer_tag: i64) -> Member {
        Member {
            can_demote: None,
            can_kick: None,
            role,
            gamer_tag,
            can_promote: None,
            scores: None,
        }
    }
}

