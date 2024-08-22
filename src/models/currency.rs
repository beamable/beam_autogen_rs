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
pub struct Currency {
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<models::CurrencyProperty>>,
    #[serde(rename = "proxy", skip_serializing_if = "Option::is_none")]
    pub proxy: Option<Box<models::FederationInfo>>,
}

impl Currency {
    pub fn new(amount: i64, id: String) -> Currency {
        Currency {
            updated_at: None,
            amount,
            id,
            properties: None,
            proxy: None,
        }
    }
}
