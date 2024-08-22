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
pub struct VipBonus {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "multiplier")]
    pub multiplier: f64,
    #[serde(rename = "roundToNearest")]
    pub round_to_nearest: i32,
}

impl VipBonus {
    pub fn new(currency: String, multiplier: f64, round_to_nearest: i32) -> VipBonus {
        VipBonus {
            currency,
            multiplier,
            round_to_nearest,
        }
    }
}

