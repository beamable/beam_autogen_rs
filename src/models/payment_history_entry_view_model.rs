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
pub struct PaymentHistoryEntryViewModel {
    #[serde(rename = "change")]
    pub change: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl PaymentHistoryEntryViewModel {
    pub fn new(change: String) -> PaymentHistoryEntryViewModel {
        PaymentHistoryEntryViewModel {
            change,
            data: None,
            timestamp: None,
        }
    }
}

