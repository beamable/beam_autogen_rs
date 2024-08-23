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
pub struct AnnouncementContentResponse {
    #[serde(rename = "content")]
    pub content: Vec<models::AnnouncementContent>,
}

impl AnnouncementContentResponse {
    pub fn new(content: Vec<models::AnnouncementContent>) -> AnnouncementContentResponse {
        AnnouncementContentResponse {
            content,
        }
    }
}

