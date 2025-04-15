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
pub struct Customer {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "activationStatus", skip_serializing_if = "Option::is_none")]
    pub activation_status: Option<String>,
    #[serde(rename = "paymentStatus", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "cid")]
    pub cid: i64,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[serde(rename = "crm_link", skip_serializing_if = "Option::is_none")]
    pub crm_link: Option<String>,
    #[serde(rename = "projects")]
    pub projects: Vec<models::Project>,
    #[serde(rename = "accounts")]
    pub accounts: Vec<models::RealmsBasicAccount>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
}

impl Customer {
    pub fn new(name: String, cid: i64, projects: Vec<models::Project>, accounts: Vec<models::RealmsBasicAccount>) -> Customer {
        Customer {
            name,
            activation_status: None,
            payment_status: None,
            image: None,
            contact: None,
            alias: None,
            cid,
            updated: None,
            crm_link: None,
            projects,
            accounts,
            created: None,
        }
    }
}

