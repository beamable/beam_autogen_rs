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
pub struct RealmRolesReport {
    #[serde(rename = "realmName")]
    pub realm_name: String,
    #[serde(rename = "realmDisplayName")]
    pub realm_display_name: String,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
}

impl RealmRolesReport {
    pub fn new(realm_name: String, realm_display_name: String, roles: Vec<String>) -> RealmRolesReport {
        RealmRolesReport {
            realm_name,
            realm_display_name,
            roles,
        }
    }
}

