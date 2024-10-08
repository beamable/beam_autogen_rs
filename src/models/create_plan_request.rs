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
pub struct CreatePlanRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "messageBusAnalytics", skip_serializing_if = "Option::is_none")]
    pub message_bus_analytics: Option<Vec<String>>,
    #[serde(rename = "mongoSrvAddress", skip_serializing_if = "Option::is_none")]
    pub mongo_srv_address: Option<String>,
    #[serde(rename = "memcachedHosts")]
    pub memcached_hosts: String,
    #[serde(rename = "mongoSSL")]
    pub mongo_ssl: bool,
    #[serde(rename = "platformJBDC")]
    pub platform_jbdc: String,
    #[serde(rename = "sharded")]
    pub sharded: bool,
    #[serde(rename = "mongoHosts")]
    pub mongo_hosts: String,
    #[serde(rename = "messageBusCommon", skip_serializing_if = "Option::is_none")]
    pub message_bus_common: Option<Vec<String>>,
    #[serde(rename = "redisShards")]
    pub redis_shards: Vec<models::RedisShardRequest>,
}

impl CreatePlanRequest {
    pub fn new(name: String, memcached_hosts: String, mongo_ssl: bool, platform_jbdc: String, sharded: bool, mongo_hosts: String, redis_shards: Vec<models::RedisShardRequest>) -> CreatePlanRequest {
        CreatePlanRequest {
            name,
            message_bus_analytics: None,
            mongo_srv_address: None,
            memcached_hosts,
            mongo_ssl,
            platform_jbdc,
            sharded,
            mongo_hosts,
            message_bus_common: None,
            redis_shards,
        }
    }
}

