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
pub struct FailedInFlightFilterRequest {
    #[serde(rename = "serviceObjectId", skip_serializing_if = "Option::is_none")]
    pub service_object_id: Option<String>,
    #[serde(rename = "serviceName")]
    pub service_name: String,
}

impl FailedInFlightFilterRequest {
    pub fn new(service_name: String) -> FailedInFlightFilterRequest {
        FailedInFlightFilterRequest {
            service_object_id: None,
            service_name,
        }
    }
}

