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
pub struct OrderRules {
    #[serde(rename = "orules")]
    pub orules: Vec<models::OrderRule>,
}

impl OrderRules {
    pub fn new(orules: Vec<models::OrderRule>) -> OrderRules {
        OrderRules {
            orules,
        }
    }
}

