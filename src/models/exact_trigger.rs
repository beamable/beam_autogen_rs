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
pub struct ExactTrigger {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "executeAt", skip_serializing_if = "Option::is_none")]
    pub execute_at: Option<String>,
}

impl ExactTrigger {
    pub fn new() -> ExactTrigger {
        ExactTrigger {
            r#type: None,
            execute_at: None,
        }
    }
}

