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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebhookServiceType {
    #[serde(rename = "UserMicroservice")]
    UserMicroservice,
    #[serde(rename = "ObjectService")]
    ObjectService,
    #[serde(rename = "BasicService")]
    BasicService,

}

impl std::fmt::Display for WebhookServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UserMicroservice => write!(f, "UserMicroservice"),
            Self::ObjectService => write!(f, "ObjectService"),
            Self::BasicService => write!(f, "BasicService"),
        }
    }
}

impl Default for WebhookServiceType {
    fn default() -> WebhookServiceType {
        Self::UserMicroservice
    }
}

