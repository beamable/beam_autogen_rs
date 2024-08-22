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
pub struct UpdateData {
    #[serde(rename = "updates", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "deletes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deletes: Option<Option<Vec<String>>>,
}

impl UpdateData {
    pub fn new() -> UpdateData {
        UpdateData {
            updates: None,
            deletes: None,
        }
    }
}

