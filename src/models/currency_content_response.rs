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
pub struct CurrencyContentResponse {
    #[serde(rename = "content")]
    pub content: Vec<models::CurrencyArchetype>,
}

impl CurrencyContentResponse {
    pub fn new(content: Vec<models::CurrencyArchetype>) -> CurrencyContentResponse {
        CurrencyContentResponse {
            content,
        }
    }
}
