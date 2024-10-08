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
pub struct CommerceLootRoll {
    #[serde(rename = "preroll")]
    pub preroll: bool,
    #[serde(rename = "externalTables", skip_serializing_if = "Option::is_none")]
    pub external_tables: Option<Vec<String>>,
}

impl CommerceLootRoll {
    pub fn new(preroll: bool) -> CommerceLootRoll {
        CommerceLootRoll {
            preroll,
            external_tables: None,
        }
    }
}

