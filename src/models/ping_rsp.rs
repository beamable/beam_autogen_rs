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
pub struct PingRsp {
    #[serde(rename = "keepAlive")]
    pub keep_alive: bool,
}

impl PingRsp {
    pub fn new(keep_alive: bool) -> PingRsp {
        PingRsp {
            keep_alive,
        }
    }
}

