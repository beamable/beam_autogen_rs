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
pub struct GroupUser {
    #[serde(rename = "inFlight", skip_serializing_if = "Option::is_none")]
    pub in_flight: Option<Vec<models::InFlightMessage>>,
    #[serde(rename = "gamerTag")]
    pub gamer_tag: i64,
    #[serde(rename = "allGroups")]
    pub all_groups: Vec<models::GroupUserMember>,
    /// Milliseconds since midnight, January 1, 1970 UTC
    #[serde(rename = "updated")]
    pub updated: i64,
    #[serde(rename = "member")]
    pub member: Box<models::GroupMemberInfo>,
    #[serde(rename = "scores", skip_serializing_if = "Option::is_none")]
    pub scores: Option<Vec<models::GroupScoreBinding>>,
}

impl GroupUser {
    pub fn new(gamer_tag: i64, all_groups: Vec<models::GroupUserMember>, updated: i64, member: models::GroupMemberInfo) -> GroupUser {
        GroupUser {
            in_flight: None,
            gamer_tag,
            all_groups,
            updated,
            member: Box::new(member),
            scores: None,
        }
    }
}

