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
pub struct CreateFederatedGameServer {
    #[serde(rename = "matchType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<Option<String>>,
}

impl CreateFederatedGameServer {
    pub fn new() -> CreateFederatedGameServer {
        CreateFederatedGameServer {
            match_type: None,
        }
    }
}

