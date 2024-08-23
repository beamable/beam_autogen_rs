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
pub struct ExternalIdentity {
    #[serde(rename = "providerService")]
    pub provider_service: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "providerNamespace")]
    pub provider_namespace: String,
}

impl ExternalIdentity {
    pub fn new(provider_service: String, user_id: String, provider_namespace: String) -> ExternalIdentity {
        ExternalIdentity {
            provider_service,
            user_id,
            provider_namespace,
        }
    }
}

