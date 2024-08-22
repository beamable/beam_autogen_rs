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
pub struct SessionClientHistoryResponse {
    #[serde(rename = "date")]
    pub date: Box<models::LocalDate>,
    #[serde(rename = "sessions")]
    pub sessions: Vec<String>,
    #[serde(rename = "installDate", skip_serializing_if = "Option::is_none")]
    pub install_date: Option<String>,
    #[serde(rename = "daysPlayed")]
    pub days_played: i32,
}

impl SessionClientHistoryResponse {
    pub fn new(date: models::LocalDate, sessions: Vec<String>, days_played: i32) -> SessionClientHistoryResponse {
        SessionClientHistoryResponse {
            date: Box::new(date),
            sessions,
            install_date: None,
            days_played,
        }
    }
}

