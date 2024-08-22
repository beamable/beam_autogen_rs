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
pub struct IsoChronology {
    #[serde(rename = "calendarType")]
    pub calendar_type: String,
    #[serde(rename = "id")]
    pub id: String,
}

impl IsoChronology {
    pub fn new(calendar_type: String, id: String) -> IsoChronology {
        IsoChronology {
            calendar_type,
            id,
        }
    }
}

