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
pub struct MicroserviceRegistrationsQuery {
    #[serde(rename = "serviceName", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "routingKey", skip_serializing_if = "Option::is_none")]
    pub routing_key: Option<String>,
    #[serde(rename = "federation", skip_serializing_if = "Option::is_none")]
    pub federation: Option<Box<models::SupportedFederation>>,
    #[serde(rename = "localOnly", skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,
}

impl MicroserviceRegistrationsQuery {
    pub fn new() -> MicroserviceRegistrationsQuery {
        MicroserviceRegistrationsQuery {
            service_name: None,
            routing_key: None,
            federation: None,
            local_only: None,
        }
    }
}
