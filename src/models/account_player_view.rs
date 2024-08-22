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
pub struct AccountPlayerView {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "deviceIds")]
    pub device_ids: Vec<String>,
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<Vec<models::ExternalIdentity>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "thirdPartyAppAssociations")]
    pub third_party_app_associations: Vec<String>,
}

impl AccountPlayerView {
    pub fn new(device_ids: Vec<String>, scopes: Vec<String>, id: i64, third_party_app_associations: Vec<String>) -> AccountPlayerView {
        AccountPlayerView {
            email: None,
            device_ids,
            scopes,
            id,
            external: None,
            language: None,
            third_party_app_associations,
        }
    }
}

