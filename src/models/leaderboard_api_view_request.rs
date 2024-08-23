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
pub struct LeaderboardApiViewRequest {
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "focus", skip_serializing_if = "Option::is_none")]
    pub focus: Option<i64>,
    #[serde(rename = "friends", skip_serializing_if = "Option::is_none")]
    pub friends: Option<bool>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    #[serde(rename = "outlier", skip_serializing_if = "Option::is_none")]
    pub outlier: Option<i64>,
    #[serde(rename = "guild", skip_serializing_if = "Option::is_none")]
    pub guild: Option<bool>,
}

impl LeaderboardApiViewRequest {
    pub fn new() -> LeaderboardApiViewRequest {
        LeaderboardApiViewRequest {
            max: None,
            focus: None,
            friends: None,
            from: None,
            outlier: None,
            guild: None,
        }
    }
}

