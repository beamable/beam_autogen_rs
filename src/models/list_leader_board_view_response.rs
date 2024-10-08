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
pub struct ListLeaderBoardViewResponse {
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "lbs")]
    pub lbs: Vec<models::LeaderBoardView>,
}

impl ListLeaderBoardViewResponse {
    pub fn new(result: String, lbs: Vec<models::LeaderBoardView>) -> ListLeaderBoardViewResponse {
        ListLeaderBoardViewResponse {
            result,
            lbs,
        }
    }
}

