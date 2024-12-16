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
pub struct MicroserviceRegistrationsResponse {
    #[serde(rename = "registrations")]
    pub registrations: Vec<models::MicroserviceRegistrations>,
}

impl MicroserviceRegistrationsResponse {
    pub fn new(registrations: Vec<models::MicroserviceRegistrations>) -> MicroserviceRegistrationsResponse {
        MicroserviceRegistrationsResponse {
            registrations,
        }
    }
}
