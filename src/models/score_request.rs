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
pub struct ScoreRequest {
    #[serde(rename = "tournamentId")]
    pub tournament_id: String,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "increment", skip_serializing_if = "Option::is_none")]
    pub increment: Option<bool>,
}

impl ScoreRequest {
    pub fn new(tournament_id: String, score: f64, player_id: i64) -> ScoreRequest {
        ScoreRequest {
            tournament_id,
            stats: None,
            score,
            player_id,
            content_id: None,
            increment: None,
        }
    }
}

