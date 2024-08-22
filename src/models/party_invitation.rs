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
pub struct PartyInvitation {
    #[serde(rename = "partyId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub party_id: Option<Option<String>>,
    #[serde(rename = "invitedBy", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<Option<String>>,
}

impl PartyInvitation {
    pub fn new() -> PartyInvitation {
        PartyInvitation {
            party_id: None,
            invited_by: None,
        }
    }
}

