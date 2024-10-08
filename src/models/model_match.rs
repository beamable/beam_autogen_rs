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
pub struct Match {
    #[serde(rename = "matchId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub match_id: Option<Option<String>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<String>>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "matchType", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<Box<models::MatchType>>,
    #[serde(rename = "teams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Option<Vec<models::Team>>>,
    #[serde(rename = "tickets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tickets: Option<Option<Vec<models::Ticket>>>,
}

impl Match {
    pub fn new() -> Match {
        Match {
            match_id: None,
            status: None,
            created: None,
            match_type: None,
            teams: None,
            tickets: None,
        }
    }
}

