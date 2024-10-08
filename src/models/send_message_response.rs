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
pub struct SendMessageResponse {
    #[serde(rename = "message")]
    pub message: Box<models::ChatV2ObjectMessage>,
}

impl SendMessageResponse {
    pub fn new(message: models::ChatV2ObjectMessage) -> SendMessageResponse {
        SendMessageResponse {
            message: Box::new(message),
        }
    }
}

