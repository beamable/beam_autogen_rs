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
pub struct EndTransactionRequest {
    #[serde(rename = "transaction")]
    pub transaction: String,
}

impl EndTransactionRequest {
    pub fn new(transaction: String) -> EndTransactionRequest {
        EndTransactionRequest {
            transaction,
        }
    }
}

