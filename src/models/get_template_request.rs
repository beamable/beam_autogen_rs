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
pub struct GetTemplateRequest {
    #[serde(rename = "templateName")]
    pub template_name: String,
    #[serde(rename = "gamerTag")]
    pub gamer_tag: i64,
}

impl GetTemplateRequest {
    pub fn new(template_name: String, gamer_tag: i64) -> GetTemplateRequest {
        GetTemplateRequest {
            template_name,
            gamer_tag,
        }
    }
}

