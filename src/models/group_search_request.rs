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
pub struct GroupSearchRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scoreMin", skip_serializing_if = "Option::is_none")]
    pub score_min: Option<i64>,
    #[serde(rename = "sortField", skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    #[serde(rename = "userScore", skip_serializing_if = "Option::is_none")]
    pub user_score: Option<i64>,
    #[serde(rename = "hasSlots", skip_serializing_if = "Option::is_none")]
    pub has_slots: Option<bool>,
    #[serde(rename = "enrollmentTypes", skip_serializing_if = "Option::is_none")]
    pub enrollment_types: Option<String>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(rename = "scoreMax", skip_serializing_if = "Option::is_none")]
    pub score_max: Option<i64>,
    #[serde(rename = "subGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<bool>,
    #[serde(rename = "sortValue", skip_serializing_if = "Option::is_none")]
    pub sort_value: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: models::GroupType,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl GroupSearchRequest {
    pub fn new(r#type: models::GroupType) -> GroupSearchRequest {
        GroupSearchRequest {
            name: None,
            score_min: None,
            sort_field: None,
            user_score: None,
            has_slots: None,
            enrollment_types: None,
            offset: None,
            score_max: None,
            sub_group: None,
            sort_value: None,
            r#type,
            limit: None,
        }
    }
}

