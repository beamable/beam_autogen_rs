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
pub struct LocalizationQuery {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
}

impl LocalizationQuery {
    pub fn new(id: String) -> LocalizationQuery {
        LocalizationQuery {
            id,
            languages: None,
        }
    }
}

