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
pub struct TicketReservationResponse {
    #[serde(rename = "tickets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tickets: Option<Option<Vec<models::Ticket>>>,
}

impl TicketReservationResponse {
    pub fn new() -> TicketReservationResponse {
        TicketReservationResponse {
            tickets: None,
        }
    }
}

