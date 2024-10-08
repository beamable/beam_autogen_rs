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
pub struct BeamoBasicReference {
    #[serde(rename = "arm")]
    pub arm: bool,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl BeamoBasicReference {
    pub fn new(arm: bool, archived: bool) -> BeamoBasicReference {
        BeamoBasicReference {
            arm,
            archived,
        }
    }
}

