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
pub struct GetS3DataRequest {
    #[serde(rename = "id")]
    pub id: i64,
}

impl GetS3DataRequest {
    pub fn new(id: i64) -> GetS3DataRequest {
        GetS3DataRequest {
            id,
        }
    }
}

