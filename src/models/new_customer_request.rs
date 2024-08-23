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
pub struct NewCustomerRequest {
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "customerName", skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(rename = "hierarchy", skip_serializing_if = "Option::is_none")]
    pub hierarchy: Option<bool>,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "password")]
    pub password: String,
}

impl NewCustomerRequest {
    pub fn new(project_name: String, email: String, password: String) -> NewCustomerRequest {
        NewCustomerRequest {
            project_name,
            email,
            customer_name: None,
            hierarchy: None,
            alias: None,
            password,
        }
    }
}

