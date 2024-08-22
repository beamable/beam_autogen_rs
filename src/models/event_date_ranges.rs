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
pub struct EventDateRanges {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "dates")]
    pub dates: Vec<models::DateRange>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl EventDateRanges {
    pub fn new(name: String, state: String, id: String, dates: Vec<models::DateRange>) -> EventDateRanges {
        EventDateRanges {
            name,
            state,
            id,
            dates,
            created_at: None,
        }
    }
}
