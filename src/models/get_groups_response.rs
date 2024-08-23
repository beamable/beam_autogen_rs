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
pub struct GetGroupsResponse {
    #[serde(rename = "entries")]
    pub entries: Vec<models::TournamentGroupEntry>,
    #[serde(rename = "focus", skip_serializing_if = "Option::is_none")]
    pub focus: Option<Box<models::TournamentGroupEntry>>,
}

impl GetGroupsResponse {
    pub fn new(entries: Vec<models::TournamentGroupEntry>) -> GetGroupsResponse {
        GetGroupsResponse {
            entries,
            focus: None,
        }
    }
}

