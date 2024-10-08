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
pub struct BeamoBasicManifestChecksums {
    #[serde(rename = "manifests")]
    pub manifests: Vec<models::BeamoBasicManifestChecksum>,
}

impl BeamoBasicManifestChecksums {
    pub fn new(manifests: Vec<models::BeamoBasicManifestChecksum>) -> BeamoBasicManifestChecksums {
        BeamoBasicManifestChecksums {
            manifests,
        }
    }
}

