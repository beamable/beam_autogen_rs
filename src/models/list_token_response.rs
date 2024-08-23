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
pub struct ListTokenResponse {
    #[serde(rename = "items")]
    pub items: Vec<models::ListTokenResponseItem>,
}

impl ListTokenResponse {
    pub fn new(items: Vec<models::ListTokenResponseItem>) -> ListTokenResponse {
        ListTokenResponse {
            items,
        }
    }
}

