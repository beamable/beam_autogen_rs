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
pub struct UploadRequestsFromPortal {
    #[serde(rename = "request")]
    pub request: Vec<models::UploadRequestFromPortal>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
}

impl UploadRequestsFromPortal {
    pub fn new(request: Vec<models::UploadRequestFromPortal>) -> UploadRequestsFromPortal {
        UploadRequestsFromPortal {
            request,
            player_id: None,
        }
    }
}

