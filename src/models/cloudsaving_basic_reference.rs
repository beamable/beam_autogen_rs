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
pub struct CloudsavingBasicReference {
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
}

impl CloudsavingBasicReference {
    pub fn new(size: i64, last_modified: i64, key: String, bucket_name: String) -> CloudsavingBasicReference {
        CloudsavingBasicReference {
            size,
            last_modified,
            e_tag: None,
            key,
            bucket_name,
        }
    }
}

