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
pub struct ScheduleDefinition {
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: Vec<String>,
    #[serde(rename = "minute")]
    pub minute: Vec<String>,
    #[serde(rename = "dayOfMonth")]
    pub day_of_month: Vec<String>,
    #[serde(rename = "year")]
    pub year: Vec<String>,
    #[serde(rename = "hour")]
    pub hour: Vec<String>,
    #[serde(rename = "second")]
    pub second: Vec<String>,
    #[serde(rename = "month")]
    pub month: Vec<String>,
}

impl ScheduleDefinition {
    pub fn new(day_of_week: Vec<String>, minute: Vec<String>, day_of_month: Vec<String>, year: Vec<String>, hour: Vec<String>, second: Vec<String>, month: Vec<String>) -> ScheduleDefinition {
        ScheduleDefinition {
            day_of_week,
            minute,
            day_of_month,
            year,
            hour,
            second,
            month,
        }
    }
}
