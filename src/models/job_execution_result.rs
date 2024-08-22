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
pub struct JobExecutionResult {
    #[serde(rename = "isSuccess", skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl JobExecutionResult {
    pub fn new() -> JobExecutionResult {
        JobExecutionResult {
            is_success: None,
            message: None,
        }
    }
}

