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
pub struct RealmConfigChangeRequest {
    #[serde(rename = "upserts", skip_serializing_if = "Option::is_none")]
    pub upserts: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "deletes", skip_serializing_if = "Option::is_none")]
    pub deletes: Option<Vec<String>>,
}

impl RealmConfigChangeRequest {
    pub fn new() -> RealmConfigChangeRequest {
        RealmConfigChangeRequest {
            upserts: None,
            deletes: None,
        }
    }
}

