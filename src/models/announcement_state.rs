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
pub struct AnnouncementState {
    #[serde(rename = "isRead")]
    pub is_read: bool,
    #[serde(rename = "isClaimed")]
    pub is_claimed: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl AnnouncementState {
    pub fn new(is_read: bool, is_claimed: bool, is_deleted: bool) -> AnnouncementState {
        AnnouncementState {
            is_read,
            is_claimed,
            is_deleted,
        }
    }
}

