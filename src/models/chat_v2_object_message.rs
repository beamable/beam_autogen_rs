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
pub struct ChatV2ObjectMessage {
    #[serde(rename = "roomId")]
    pub room_id: String,
    #[serde(rename = "gamerTag")]
    pub gamer_tag: i64,
    #[serde(rename = "reactions")]
    pub reactions: std::collections::HashMap<String, String>,
    #[serde(rename = "timestampMillis")]
    pub timestamp_millis: i64,
    #[serde(rename = "censoredContent")]
    pub censored_content: String,
    #[serde(rename = "messageId")]
    pub message_id: uuid::Uuid,
    #[serde(rename = "content")]
    pub content: String,
}

impl ChatV2ObjectMessage {
    pub fn new(room_id: String, gamer_tag: i64, reactions: std::collections::HashMap<String, String>, timestamp_millis: i64, censored_content: String, message_id: uuid::Uuid, content: String) -> ChatV2ObjectMessage {
        ChatV2ObjectMessage {
            room_id,
            gamer_tag,
            reactions,
            timestamp_millis,
            censored_content,
            message_id,
            content,
        }
    }
}

