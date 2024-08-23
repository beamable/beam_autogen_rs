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
pub struct CreateTrialRestRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "cohorts")]
    pub cohorts: String,
    #[serde(rename = "cohortType")]
    pub cohort_type: String,
    #[serde(rename = "strat")]
    pub strat: String,
    #[serde(rename = "customRules", skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<models::TrialCustomRule>>,
}

impl CreateTrialRestRequest {
    pub fn new(name: String, cohorts: String, cohort_type: String, strat: String) -> CreateTrialRestRequest {
        CreateTrialRestRequest {
            name,
            cohorts,
            cohort_type,
            strat,
            custom_rules: None,
        }
    }
}

