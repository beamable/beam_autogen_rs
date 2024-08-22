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
pub struct ContentReference {
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "lastChanged", skip_serializing_if = "Option::is_none")]
    pub last_changed: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "visibility")]
    pub visibility: models::ContentVisibility,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
}

impl ContentReference {
    pub fn new(tag: String, tags: Vec<String>, uri: String, version: String, id: String, r#type: Type, visibility: models::ContentVisibility) -> ContentReference {
        ContentReference {
            tag,
            tags,
            uri,
            version,
            id,
            checksum: None,
            last_changed: None,
            r#type,
            visibility,
            created: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "content")]
    Content,
}

impl Default for Type {
    fn default() -> Type {
        Self::Content
    }
}

