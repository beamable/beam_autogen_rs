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
pub struct PublishMessage {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "persist", skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,
    #[serde(rename = "headers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Option<std::collections::HashMap<String, String>>>,
}

impl PublishMessage {
    pub fn new() -> PublishMessage {
        PublishMessage {
            r#type: None,
            topic: None,
            message: None,
            persist: None,
            headers: None,
        }
    }
}

