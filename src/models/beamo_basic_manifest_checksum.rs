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
pub struct BeamoBasicManifestChecksum {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "checksum")]
    pub checksum: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

impl BeamoBasicManifestChecksum {
    pub fn new(id: String, checksum: String, created_at: i64) -> BeamoBasicManifestChecksum {
        BeamoBasicManifestChecksum {
            id,
            checksum,
            created_at,
        }
    }
}

