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
pub struct Cohort {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "customRule", skip_serializing_if = "Option::is_none")]
    pub custom_rule: Option<Vec<models::CustomCohortRule>>,
    #[serde(rename = "populationCap", skip_serializing_if = "Option::is_none")]
    pub population_cap: Option<i64>,
    #[serde(rename = "assigned")]
    pub assigned: i64,
    #[serde(rename = "pct", skip_serializing_if = "Option::is_none")]
    pub pct: Option<i32>,
    #[serde(rename = "cloudData", skip_serializing_if = "Option::is_none")]
    pub cloud_data: Option<Vec<models::CloudStorage>>,
}

impl Cohort {
    pub fn new(name: String, assigned: i64) -> Cohort {
        Cohort {
            name,
            custom_rule: None,
            population_cap: None,
            assigned,
            pct: None,
            cloud_data: None,
        }
    }
}

