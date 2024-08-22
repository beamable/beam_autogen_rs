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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LobbyRestriction {
    #[serde(rename = "Null")]
    Null,
    #[serde(rename = "Closed")]
    Closed,
    #[serde(rename = "Open")]
    Open,

}

impl std::fmt::Display for LobbyRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::Closed => write!(f, "Closed"),
            Self::Open => write!(f, "Open"),
        }
    }
}

impl Default for LobbyRestriction {
    fn default() -> LobbyRestriction {
        Self::Null
    }
}

