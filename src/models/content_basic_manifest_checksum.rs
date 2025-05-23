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
pub struct ContentBasicManifestChecksum {
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "checksum")]
    pub checksum: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl ContentBasicManifestChecksum {
    pub fn new(id: String, checksum: String, created_at: i64) -> ContentBasicManifestChecksum {
        ContentBasicManifestChecksum {
            archived: None,
            id,
            checksum,
            created_at,
            uid: None,
        }
    }
}

