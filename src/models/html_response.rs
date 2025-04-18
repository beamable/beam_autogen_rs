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
pub struct HtmlResponse {
    #[serde(rename = "html")]
    pub html: String,
}

impl HtmlResponse {
    pub fn new(html: String) -> HtmlResponse {
        HtmlResponse {
            html,
        }
    }
}

