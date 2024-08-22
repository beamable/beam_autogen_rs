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
pub struct AccountRegistration {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl AccountRegistration {
    pub fn new(email: String, password: String) -> AccountRegistration {
        AccountRegistration {
            email,
            password,
        }
    }
}

