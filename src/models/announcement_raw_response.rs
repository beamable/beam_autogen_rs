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
pub struct AnnouncementRawResponse {
    #[serde(rename = "announcements")]
    pub announcements: std::collections::HashMap<String, models::AnnouncementState>,
}

impl AnnouncementRawResponse {
    pub fn new(announcements: std::collections::HashMap<String, models::AnnouncementState>) -> AnnouncementRawResponse {
        AnnouncementRawResponse {
            announcements,
        }
    }
}

