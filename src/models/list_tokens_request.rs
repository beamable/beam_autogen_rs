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
pub struct ListTokensRequest {
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<i64>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(rename = "gamerTagOrAccountId")]
    pub gamer_tag_or_account_id: i64,
}

impl ListTokensRequest {
    pub fn new(page_size: i32, page: i32, gamer_tag_or_account_id: i64) -> ListTokensRequest {
        ListTokensRequest {
            page_size,
            page,
            cid: None,
            pid: None,
            gamer_tag_or_account_id,
        }
    }
}

