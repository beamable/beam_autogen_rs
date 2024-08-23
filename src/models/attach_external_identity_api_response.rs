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
pub struct AttachExternalIdentityApiResponse {
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "challenge_token", skip_serializing_if = "Option::is_none")]
    pub challenge_token: Option<String>,
}

impl AttachExternalIdentityApiResponse {
    pub fn new(result: String) -> AttachExternalIdentityApiResponse {
        AttachExternalIdentityApiResponse {
            result,
            challenge_token: None,
        }
    }
}

