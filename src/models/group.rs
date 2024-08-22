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
pub struct Group {
    #[serde(rename = "inFlight", skip_serializing_if = "Option::is_none")]
    pub in_flight: Option<Vec<models::InFlightMessage>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "enrollmentType")]
    pub enrollment_type: String,
    #[serde(rename = "donations", skip_serializing_if = "Option::is_none")]
    pub donations: Option<Vec<models::DonationRequest>>,
    #[serde(rename = "freeSlots")]
    pub free_slots: i32,
    #[serde(rename = "maybeDonations", skip_serializing_if = "Option::is_none")]
    pub maybe_donations: Option<std::collections::HashMap<String, models::DonationRequest>>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "canUpdateMOTD", skip_serializing_if = "Option::is_none")]
    pub can_update_motd: Option<bool>,
    #[serde(rename = "shard", skip_serializing_if = "Option::is_none")]
    pub shard: Option<String>,
    #[serde(rename = "canUpdateSlogan", skip_serializing_if = "Option::is_none")]
    pub can_update_slogan: Option<bool>,
    #[serde(rename = "leader")]
    pub leader: i64,
    #[serde(rename = "slogan")]
    pub slogan: String,
    #[serde(rename = "requirement")]
    pub requirement: i64,
    #[serde(rename = "motd")]
    pub motd: String,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "clientData", skip_serializing_if = "Option::is_none")]
    pub client_data: Option<String>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<models::GroupRole>>,
    #[serde(rename = "scores")]
    pub scores: std::collections::HashMap<String, String>,
    #[serde(rename = "canUpdateEnrollment", skip_serializing_if = "Option::is_none")]
    pub can_update_enrollment: Option<bool>,
    #[serde(rename = "members")]
    pub members: Vec<models::Member>,
    #[serde(rename = "canDisband", skip_serializing_if = "Option::is_none")]
    pub can_disband: Option<bool>,
    #[serde(rename = "type")]
    pub r#type: models::GroupType,
    #[serde(rename = "maxSize")]
    pub max_size: i32,
    #[serde(rename = "subGroups")]
    pub sub_groups: Vec<models::Group>,
    /// Milliseconds since midnight, January 1, 1970 UTC
    #[serde(rename = "created")]
    pub created: i64,
}

impl Group {
    pub fn new(name: String, enrollment_type: String, free_slots: i32, leader: i64, slogan: String, requirement: i64, motd: String, id: i64, scores: std::collections::HashMap<String, String>, members: Vec<models::Member>, r#type: models::GroupType, max_size: i32, sub_groups: Vec<models::Group>, created: i64) -> Group {
        Group {
            in_flight: None,
            name,
            enrollment_type,
            donations: None,
            free_slots,
            maybe_donations: None,
            tag: None,
            can_update_motd: None,
            shard: None,
            can_update_slogan: None,
            leader,
            slogan,
            requirement,
            motd,
            version: None,
            id,
            client_data: None,
            roles: None,
            scores,
            can_update_enrollment: None,
            members,
            can_disband: None,
            r#type,
            max_size,
            sub_groups,
            created,
        }
    }
}
