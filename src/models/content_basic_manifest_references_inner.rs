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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentBasicManifestReferencesInner {
    ContentReference(Box<models::ContentReference>),
    TextReference(Box<models::TextReference>),
    BinaryReference(Box<models::BinaryReference>),
}

impl Default for ContentBasicManifestReferencesInner {
    fn default() -> Self {
        Self::ContentReference(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "binary")]
    Binary,
}

impl Default for Type {
    fn default() -> Type {
        Self::Content
    }
}

