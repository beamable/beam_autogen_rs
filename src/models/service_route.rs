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
pub struct ServiceRoute {
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "serviceTypeStr")]
    pub service_type_str: models::WebhookServiceType,
}

impl ServiceRoute {
    pub fn new(service: String, endpoint: String, service_type_str: models::WebhookServiceType) -> ServiceRoute {
        ServiceRoute {
            service,
            endpoint,
            service_type_str,
        }
    }
}

