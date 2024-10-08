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
pub struct ServicePlan {
    #[serde(rename = "minCustomerStatusSaved", skip_serializing_if = "Option::is_none")]
    pub min_customer_status_saved: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "dataDomain")]
    pub data_domain: Box<models::DataDomain>,
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Box<models::ServiceLimits>>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
}

impl ServicePlan {
    pub fn new(name: String, data_domain: models::DataDomain) -> ServicePlan {
        ServicePlan {
            min_customer_status_saved: None,
            name,
            data_domain: Box::new(data_domain),
            limits: None,
            created: None,
        }
    }
}

