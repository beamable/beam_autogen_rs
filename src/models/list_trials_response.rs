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
pub struct ListTrialsResponse {
    #[serde(rename = "result")]
    pub result: Vec<models::Trial>,
}

impl ListTrialsResponse {
    pub fn new(result: Vec<models::Trial>) -> ListTrialsResponse {
        ListTrialsResponse {
            result,
        }
    }
}

