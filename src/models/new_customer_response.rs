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
pub struct NewCustomerResponse {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "activationPending")]
    pub activation_pending: bool,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "cid")]
    pub cid: i64,
    #[serde(rename = "pid")]
    pub pid: String,
    #[serde(rename = "token")]
    pub token: Box<models::TokenResponse>,
}

impl NewCustomerResponse {
    pub fn new(name: String, project_name: String, activation_pending: bool, cid: i64, pid: String, token: models::TokenResponse) -> NewCustomerResponse {
        NewCustomerResponse {
            name,
            project_name,
            activation_pending,
            alias: None,
            cid,
            pid,
            token: Box::new(token),
        }
    }
}

