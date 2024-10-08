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
pub struct TeamContentProto {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "maxPlayers", skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i32>,
    #[serde(rename = "minPlayers", skip_serializing_if = "Option::is_none")]
    pub min_players: Option<i32>,
}

impl TeamContentProto {
    pub fn new() -> TeamContentProto {
        TeamContentProto {
            name: None,
            max_players: None,
            min_players: None,
        }
    }
}

