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
pub struct SteamOrderInfoItem {
    #[serde(rename = "vat")]
    pub vat: i64,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "itemid")]
    pub itemid: i64,
    #[serde(rename = "qty")]
    pub qty: i64,
    #[serde(rename = "itemstatus")]
    pub itemstatus: String,
}

impl SteamOrderInfoItem {
    pub fn new(vat: i64, amount: i64, itemid: i64, qty: i64, itemstatus: String) -> SteamOrderInfoItem {
        SteamOrderInfoItem {
            vat,
            amount,
            itemid,
            qty,
            itemstatus,
        }
    }
}

