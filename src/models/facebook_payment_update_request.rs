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
pub struct FacebookPaymentUpdateRequest {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "entry")]
    pub entry: Vec<models::FacebookUpdatedEntry>,
}

impl FacebookPaymentUpdateRequest {
    pub fn new(object: String, entry: Vec<models::FacebookUpdatedEntry>) -> FacebookPaymentUpdateRequest {
        FacebookPaymentUpdateRequest {
            object,
            entry,
        }
    }
}

