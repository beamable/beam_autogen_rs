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
pub struct FailPurchaseRequest {
    #[serde(rename = "txid")]
    pub txid: i64,
    #[serde(rename = "reason")]
    pub reason: String,
}

impl FailPurchaseRequest {
    pub fn new(txid: i64, reason: String) -> FailPurchaseRequest {
        FailPurchaseRequest {
            txid,
            reason,
        }
    }
}
