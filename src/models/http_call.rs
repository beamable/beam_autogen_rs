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
pub struct HttpCall {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "headers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Option<Vec<models::StringStringKeyValuePair>>>,
    #[serde(rename = "body", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub body: Option<Option<String>>,
    #[serde(rename = "contentType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<Option<String>>,
}

impl HttpCall {
    pub fn new() -> HttpCall {
        HttpCall {
            r#type: None,
            uri: None,
            method: None,
            headers: None,
            body: None,
            content_type: None,
        }
    }
}

