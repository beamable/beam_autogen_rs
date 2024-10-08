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
pub struct LocalizedPriceMap {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "prices")]
    pub prices: Vec<models::LocalizedPrice>,
}

impl LocalizedPriceMap {
    pub fn new(currency: String, prices: Vec<models::LocalizedPrice>) -> LocalizedPriceMap {
        LocalizedPriceMap {
            currency,
            prices,
        }
    }
}

