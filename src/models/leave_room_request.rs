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
pub struct LeaveRoomRequest {
    #[serde(rename = "roomId")]
    pub room_id: String,
}

impl LeaveRoomRequest {
    pub fn new(room_id: String) -> LeaveRoomRequest {
        LeaveRoomRequest {
            room_id,
        }
    }
}

