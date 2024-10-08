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
pub struct OnlineStatusQuery {
    #[serde(rename = "playerIds", skip_serializing_if = "Option::is_none")]
    pub player_ids: Option<Vec<String>>,
}

impl OnlineStatusQuery {
    pub fn new() -> OnlineStatusQuery {
        OnlineStatusQuery {
            player_ids: None,
        }
    }
}

