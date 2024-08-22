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
pub struct GetMetricsUrlRequest {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
}

impl GetMetricsUrlRequest {
    pub fn new(service_name: String, metric_name: String) -> GetMetricsUrlRequest {
        GetMetricsUrlRequest {
            start_time: None,
            service_name,
            metric_name,
            end_time: None,
            period: None,
        }
    }
}
