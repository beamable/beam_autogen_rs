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
pub struct StartSessionResponse {
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "gamer", skip_serializing_if = "Option::is_none")]
    pub gamer: Option<Box<models::GamerTag>>,
}

impl StartSessionResponse {
    pub fn new(result: String) -> StartSessionResponse {
        StartSessionResponse {
            result,
            gamer: None,
        }
    }
}

