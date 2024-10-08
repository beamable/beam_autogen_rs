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
pub struct JoinRequest {
    #[serde(rename = "tournamentId")]
    pub tournament_id: String,
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
}

impl JoinRequest {
    pub fn new(tournament_id: String) -> JoinRequest {
        JoinRequest {
            tournament_id,
            content_id: None,
        }
    }
}

