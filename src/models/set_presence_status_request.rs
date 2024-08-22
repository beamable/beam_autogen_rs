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
pub struct SetPresenceStatusRequest {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::PresenceStatus>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl SetPresenceStatusRequest {
    pub fn new() -> SetPresenceStatusRequest {
        SetPresenceStatusRequest {
            status: None,
            description: None,
        }
    }
}
