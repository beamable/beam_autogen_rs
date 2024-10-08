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
pub struct DeleteThirdPartyAssociation {
    #[serde(rename = "thirdParty")]
    pub third_party: String,
    #[serde(rename = "userAppId")]
    pub user_app_id: String,
}

impl DeleteThirdPartyAssociation {
    pub fn new(third_party: String, user_app_id: String) -> DeleteThirdPartyAssociation {
        DeleteThirdPartyAssociation {
            third_party,
            user_app_id,
        }
    }
}

