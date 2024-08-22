/*
 * Auth Actor
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeaderBoardView {
    #[serde(rename = "lbId")]
    pub lb_id: String,
    #[serde(rename = "boardSize")]
    pub board_size: i64,
    #[serde(rename = "rankgt", skip_serializing_if = "Option::is_none")]
    pub rankgt: Option<Box<models::RankEntry>>,
    #[serde(rename = "rankings")]
    pub rankings: Vec<models::RankEntry>,
}

impl LeaderBoardView {
    pub fn new(lb_id: String, board_size: i64, rankings: Vec<models::RankEntry>) -> LeaderBoardView {
        LeaderBoardView {
            lb_id,
            board_size,
            rankgt: None,
            rankings,
        }
    }
}
