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
pub struct GetSocialStatusesRequest {
    #[serde(rename = "playerIds")]
    pub player_ids: Vec<String>,
}

impl GetSocialStatusesRequest {
    pub fn new(player_ids: Vec<String>) -> GetSocialStatusesRequest {
        GetSocialStatusesRequest {
            player_ids,
        }
    }
}

