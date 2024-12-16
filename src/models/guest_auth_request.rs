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
pub struct GuestAuthRequest {
    #[serde(rename = "scope", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Option<String>>,
    #[serde(rename = "customerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Option<String>>,
    #[serde(rename = "realmId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub realm_id: Option<Option<String>>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<models::AuthActorContextInfo>>,
    #[serde(rename = "initProperties", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub init_properties: Option<Option<std::collections::HashMap<String, String>>>,
}

impl GuestAuthRequest {
    pub fn new() -> GuestAuthRequest {
        GuestAuthRequest {
            scope: None,
            customer_id: None,
            realm_id: None,
            context: None,
            init_properties: None,
        }
    }
}
