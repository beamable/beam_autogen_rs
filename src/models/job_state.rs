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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobState {
    #[serde(rename = "ENQUEUED")]
    Enqueued,
    #[serde(rename = "DISPATCHED")]
    Dispatched,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "ERROR")]
    Error,

}

impl std::fmt::Display for JobState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Enqueued => write!(f, "ENQUEUED"),
            Self::Dispatched => write!(f, "DISPATCHED"),
            Self::Running => write!(f, "RUNNING"),
            Self::Done => write!(f, "DONE"),
            Self::Canceled => write!(f, "CANCELED"),
            Self::Suspended => write!(f, "SUSPENDED"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

impl Default for JobState {
    fn default() -> JobState {
        Self::Enqueued
    }
}

