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
pub struct PaymentAuditEntryViewModel {
    #[serde(rename = "providerid")]
    pub providerid: String,
    #[serde(rename = "history")]
    pub history: Vec<models::PaymentHistoryEntryViewModel>,
    #[serde(rename = "txid")]
    pub txid: i64,
    #[serde(rename = "providername")]
    pub providername: String,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "obtainItems", skip_serializing_if = "Option::is_none")]
    pub obtain_items: Option<Vec<models::ItemCreateRequest>>,
    #[serde(rename = "txstate")]
    pub txstate: String,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[serde(rename = "obtainCurrency", skip_serializing_if = "Option::is_none")]
    pub obtain_currency: Option<Vec<models::CurrencyChange>>,
    #[serde(rename = "entitlements")]
    pub entitlements: Vec<models::EntitlementGenerator>,
    #[serde(rename = "details")]
    pub details: Box<models::PaymentDetailsEntryViewModel>,
    #[serde(rename = "replayGuardValue", skip_serializing_if = "Option::is_none")]
    pub replay_guard_value: Option<String>,
    #[serde(rename = "gt")]
    pub gt: i64,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
}

impl PaymentAuditEntryViewModel {
    pub fn new(providerid: String, history: Vec<models::PaymentHistoryEntryViewModel>, txid: i64, providername: String, txstate: String, entitlements: Vec<models::EntitlementGenerator>, details: models::PaymentDetailsEntryViewModel, gt: i64) -> PaymentAuditEntryViewModel {
        PaymentAuditEntryViewModel {
            providerid,
            history,
            txid,
            providername,
            version: None,
            obtain_items: None,
            txstate,
            updated: None,
            obtain_currency: None,
            entitlements,
            details: Box::new(details),
            replay_guard_value: None,
            gt,
            created: None,
        }
    }
}

