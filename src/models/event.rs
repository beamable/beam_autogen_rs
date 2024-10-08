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
pub struct Event {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "phases")]
    pub phases: Vec<models::EventPhase>,
    #[serde(rename = "partition_size", skip_serializing_if = "Option::is_none")]
    pub partition_size: Option<i32>,
    #[serde(rename = "group_rewards", skip_serializing_if = "Option::is_none")]
    pub group_rewards: Option<Box<models::EventGroupRewards>>,
    #[serde(rename = "cohortSettings", skip_serializing_if = "Option::is_none")]
    pub cohort_settings: Option<Box<models::LeaderboardCohortSettings>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::ClientPermission>>,
    #[serde(rename = "stores", skip_serializing_if = "Option::is_none")]
    pub stores: Option<Vec<String>>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "score_rewards", skip_serializing_if = "Option::is_none")]
    pub score_rewards: Option<Vec<models::EventRewardContent>>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<models::Schedule>>,
    #[serde(rename = "rank_rewards", skip_serializing_if = "Option::is_none")]
    pub rank_rewards: Option<Vec<models::EventRewardContent>>,
}

impl Event {
    pub fn new(name: String, start_date: String, phases: Vec<models::EventPhase>, symbol: String) -> Event {
        Event {
            name,
            start_date,
            phases,
            partition_size: None,
            group_rewards: None,
            cohort_settings: None,
            permissions: None,
            stores: None,
            symbol,
            score_rewards: None,
            schedule: None,
            rank_rewards: None,
        }
    }
}

