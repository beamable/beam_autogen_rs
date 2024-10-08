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
pub struct CustomersResponse {
    #[serde(rename = "result")]
    pub result: Vec<models::Customer>,
}

impl CustomersResponse {
    pub fn new(result: Vec<models::Customer>) -> CustomersResponse {
        CustomersResponse {
            result,
        }
    }
}

