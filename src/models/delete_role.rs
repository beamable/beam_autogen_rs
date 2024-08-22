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
pub struct DeleteRole {
    #[serde(rename = "realm", skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl DeleteRole {
    pub fn new() -> DeleteRole {
        DeleteRole {
            realm: None,
            role: None,
        }
    }
}

