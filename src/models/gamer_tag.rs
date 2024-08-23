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
pub struct GamerTag {
    #[serde(rename = "tag")]
    pub tag: i64,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<i64>,
    #[serde(rename = "trials", skip_serializing_if = "Option::is_none")]
    pub trials: Option<Vec<models::CohortEntry>>,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::SessionUser>>,
}

impl GamerTag {
    pub fn new(tag: i64, platform: String) -> GamerTag {
        GamerTag {
            tag,
            alias: None,
            added: None,
            trials: None,
            platform,
            user: None,
        }
    }
}

