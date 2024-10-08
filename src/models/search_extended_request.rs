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
pub struct SearchExtendedRequest {
    #[serde(rename = "criteria")]
    pub criteria: Vec<models::StatsSearchCriteria>,
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(rename = "statKeys")]
    pub stat_keys: Vec<String>,
    #[serde(rename = "access")]
    pub access: String,
}

impl SearchExtendedRequest {
    pub fn new(criteria: Vec<models::StatsSearchCriteria>, domain: String, object_type: String, stat_keys: Vec<String>, access: String) -> SearchExtendedRequest {
        SearchExtendedRequest {
            criteria,
            domain,
            object_type,
            stat_keys,
            access,
        }
    }
}

