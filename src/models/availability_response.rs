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
pub struct AvailabilityResponse {
    #[serde(rename = "name")]
    pub name: bool,
    #[serde(rename = "tag")]
    pub tag: bool,
}

impl AvailabilityResponse {
    pub fn new(name: bool, tag: bool) -> AvailabilityResponse {
        AvailabilityResponse {
            name,
            tag,
        }
    }
}

