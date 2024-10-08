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
pub struct UpdateLobby {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "restriction", skip_serializing_if = "Option::is_none")]
    pub restriction: Option<models::LobbyRestriction>,
    #[serde(rename = "matchType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<Option<String>>,
    #[serde(rename = "maxPlayers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_players: Option<Option<i32>>,
    #[serde(rename = "newHost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new_host: Option<Option<String>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::UpdateData>>,
}

impl UpdateLobby {
    pub fn new() -> UpdateLobby {
        UpdateLobby {
            name: None,
            description: None,
            restriction: None,
            match_type: None,
            max_players: None,
            new_host: None,
            data: None,
        }
    }
}

