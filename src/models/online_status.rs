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
pub struct OnlineStatus {
    #[serde(rename = "online", skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
    #[serde(rename = "lastOnline", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_online: Option<Option<String>>,
    #[serde(rename = "playerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::PresenceStatus>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl OnlineStatus {
    pub fn new() -> OnlineStatus {
        OnlineStatus {
            online: None,
            last_online: None,
            player_id: None,
            status: None,
            description: None,
        }
    }
}

