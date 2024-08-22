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
pub struct SendMailRequest {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "playerRewards", skip_serializing_if = "Option::is_none")]
    pub player_rewards: Option<Box<models::PlayerReward>>,
    #[serde(rename = "receiverGamerTag")]
    pub receiver_gamer_tag: i64,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Box<models::MailRewards>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "senderGamerTag")]
    pub sender_gamer_tag: i64,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "bodyRef", skip_serializing_if = "Option::is_none")]
    pub body_ref: Option<i64>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<models::AttachmentRequest>>,
}

impl SendMailRequest {
    pub fn new(receiver_gamer_tag: i64, sender_gamer_tag: i64, category: String) -> SendMailRequest {
        SendMailRequest {
            body: None,
            expires: None,
            player_rewards: None,
            receiver_gamer_tag,
            subject: None,
            rewards: None,
            id: None,
            sender_gamer_tag,
            category,
            body_ref: None,
            attachments: None,
        }
    }
}
