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
pub struct StatsSearchRequest {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "access")]
    pub access: String,
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(rename = "criteria")]
    pub criteria: Vec<models::StatsSearchCriteria>,
}

impl StatsSearchRequest {
    pub fn new(domain: String, access: String, object_type: String, criteria: Vec<models::StatsSearchCriteria>) -> StatsSearchRequest {
        StatsSearchRequest {
            domain,
            access,
            object_type,
            criteria,
        }
    }
}

