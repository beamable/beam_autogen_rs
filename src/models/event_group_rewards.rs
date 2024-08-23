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
pub struct EventGroupRewards {
    #[serde(rename = "scoreRewards", skip_serializing_if = "Option::is_none")]
    pub score_rewards: Option<Vec<models::EventRewardContent>>,
}

impl EventGroupRewards {
    pub fn new() -> EventGroupRewards {
        EventGroupRewards {
            score_rewards: None,
        }
    }
}

