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
pub struct GetTotalCouponResponse {
    #[serde(rename = "count")]
    pub count: i64,
}

impl GetTotalCouponResponse {
    pub fn new(count: i64) -> GetTotalCouponResponse {
        GetTotalCouponResponse {
            count,
        }
    }
}

