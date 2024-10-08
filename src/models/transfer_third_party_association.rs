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
pub struct TransferThirdPartyAssociation {
    #[serde(rename = "fromAccountId")]
    pub from_account_id: i64,
    #[serde(rename = "thirdParty")]
    pub third_party: Box<models::ThirdPartyAssociation>,
}

impl TransferThirdPartyAssociation {
    pub fn new(from_account_id: i64, third_party: models::ThirdPartyAssociation) -> TransferThirdPartyAssociation {
        TransferThirdPartyAssociation {
            from_account_id,
            third_party: Box::new(third_party),
        }
    }
}

