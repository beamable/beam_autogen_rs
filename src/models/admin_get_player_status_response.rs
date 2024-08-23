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
pub struct AdminGetPlayerStatusResponse {
    #[serde(rename = "statuses")]
    pub statuses: Vec<models::AdminPlayerStatus>,
}

impl AdminGetPlayerStatusResponse {
    pub fn new(statuses: Vec<models::AdminPlayerStatus>) -> AdminGetPlayerStatusResponse {
        AdminGetPlayerStatusResponse {
            statuses,
        }
    }
}

