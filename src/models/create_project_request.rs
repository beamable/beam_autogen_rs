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
pub struct CreateProjectRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "sharded", skip_serializing_if = "Option::is_none")]
    pub sharded: Option<bool>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}

impl CreateProjectRequest {
    pub fn new(name: String) -> CreateProjectRequest {
        CreateProjectRequest {
            name,
            plan: None,
            sharded: None,
            parent: None,
        }
    }
}

