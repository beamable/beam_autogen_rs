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
pub struct Message {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Milliseconds since midnight, January 1, 1970 UTC
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    #[serde(rename = "playerRewards", skip_serializing_if = "Option::is_none")]
    pub player_rewards: Option<Box<models::PlayerReward>>,
    #[serde(rename = "receiverGamerTag")]
    pub receiver_gamer_tag: i64,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Box<models::MailRewards>>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "senderGamerTag")]
    pub sender_gamer_tag: i64,
    /// Milliseconds since midnight, January 1, 1970 UTC
    #[serde(rename = "sent")]
    pub sent: i64,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "bodyRef", skip_serializing_if = "Option::is_none")]
    pub body_ref: Option<i64>,
    #[serde(rename = "attachments")]
    pub attachments: Vec<models::Attachment>,
    #[serde(rename = "claimedTimeMs", skip_serializing_if = "Option::is_none")]
    pub claimed_time_ms: Option<i64>,
}

impl Message {
    pub fn new(receiver_gamer_tag: i64, state: String, id: i64, sender_gamer_tag: i64, sent: i64, category: String, attachments: Vec<models::Attachment>) -> Message {
        Message {
            body: None,
            expires: None,
            player_rewards: None,
            receiver_gamer_tag,
            subject: None,
            state,
            rewards: None,
            id,
            sender_gamer_tag,
            sent,
            category,
            body_ref: None,
            attachments,
            claimed_time_ms: None,
        }
    }
}

