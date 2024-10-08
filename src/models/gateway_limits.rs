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
pub struct GatewayLimits {
    #[serde(rename = "maxConcurrentRequests")]
    pub max_concurrent_requests: i32,
}

impl GatewayLimits {
    pub fn new(max_concurrent_requests: i32) -> GatewayLimits {
        GatewayLimits {
            max_concurrent_requests,
        }
    }
}

