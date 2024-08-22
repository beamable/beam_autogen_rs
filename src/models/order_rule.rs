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
pub struct OrderRule {
    #[serde(rename = "v")]
    pub v: String,
    #[serde(rename = "o")]
    pub o: i32,
}

impl OrderRule {
    pub fn new(v: String, o: i32) -> OrderRule {
        OrderRule {
            v,
            o,
        }
    }
}
