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
pub struct GetS3DataResponse {
    #[serde(rename = "data")]
    pub data: Vec<i32>,
}

impl GetS3DataResponse {
    pub fn new(data: Vec<i32>) -> GetS3DataResponse {
        GetS3DataResponse {
            data,
        }
    }
}
