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
pub struct ListMailCategoriesResponse {
    #[serde(rename = "result")]
    pub result: Vec<String>,
}

impl ListMailCategoriesResponse {
    pub fn new(result: Vec<String>) -> ListMailCategoriesResponse {
        ListMailCategoriesResponse {
            result,
        }
    }
}

