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
pub struct EmailUpdateConfirmation {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl EmailUpdateConfirmation {
    pub fn new(code: String, password: String) -> EmailUpdateConfirmation {
        EmailUpdateConfirmation {
            code,
            password,
        }
    }
}
