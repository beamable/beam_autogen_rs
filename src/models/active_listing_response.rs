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
pub struct ActiveListingResponse {
    #[serde(rename = "storeSymbol")]
    pub store_symbol: String,
    #[serde(rename = "listing")]
    pub listing: Box<models::PlayerListingView>,
}

impl ActiveListingResponse {
    pub fn new(store_symbol: String, listing: models::PlayerListingView) -> ActiveListingResponse {
        ActiveListingResponse {
            store_symbol,
            listing: Box::new(listing),
        }
    }
}

