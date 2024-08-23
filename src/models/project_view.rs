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
pub struct ProjectView {
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<i64>,
    #[serde(rename = "pid")]
    pub pid: String,
    #[serde(rename = "sharded", skip_serializing_if = "Option::is_none")]
    pub sharded: Option<bool>,
}

impl ProjectView {
    pub fn new(project_name: String, pid: String) -> ProjectView {
        ProjectView {
            secret: None,
            parent: None,
            children: None,
            project_name,
            archived: None,
            cid: None,
            pid,
            sharded: None,
        }
    }
}

