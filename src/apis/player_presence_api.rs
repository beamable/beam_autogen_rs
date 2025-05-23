/*
 * Beamable API
 *
 * Autogenerated Beamable API
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};

/// struct for passing parameters to the method [`api_players_player_id_presence_get`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdPresenceGetParams {
    pub player_id: String,
    /// Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token
    pub x_beam_scope: Option<String>,
    /// Override the playerId of the requester. This is only necessary when not using a JWT bearer token.
    pub x_beam_gamertag: Option<String>
}

/// struct for passing parameters to the method [`api_players_player_id_presence_put`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdPresencePutParams {
    pub player_id: String,
    /// Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token
    pub x_beam_scope: Option<String>,
    /// Override the playerId of the requester. This is only necessary when not using a JWT bearer token.
    pub x_beam_gamertag: Option<String>
}

/// struct for passing parameters to the method [`api_players_player_id_presence_status_put`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdPresenceStatusPutParams {
    pub player_id: String,
    /// Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token
    pub x_beam_scope: Option<String>,
    /// Override the playerId of the requester. This is only necessary when not using a JWT bearer token.
    pub x_beam_gamertag: Option<String>,
    pub set_presence_status_request: Option<models::SetPresenceStatusRequest>
}


/// struct for typed errors of method [`api_players_player_id_presence_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdPresenceGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_players_player_id_presence_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdPresencePutError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_players_player_id_presence_status_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdPresenceStatusPutError {
    UnknownValue(serde_json::Value),
}


pub async fn api_players_player_id_presence_get(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdPresenceGetParams) -> Result<models::OnlineStatus, Error<ApiPlayersPlayerIdPresenceGetError>> {

    let uri_str = format!("{}/api/players/{playerId}/presence", configuration.base_path, playerId=crate::apis::urlencode(params.player_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_beam_scope {
        req_builder = req_builder.header("X-BEAM-SCOPE", param_value.to_string());
    }
    if let Some(param_value) = params.x_beam_gamertag {
        req_builder = req_builder.header("X-BEAM-GAMERTAG", param_value.to_string());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OnlineStatus`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OnlineStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiPlayersPlayerIdPresenceGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn api_players_player_id_presence_put(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdPresencePutParams) -> Result<serde_json::Value, Error<ApiPlayersPlayerIdPresencePutError>> {

    let uri_str = format!("{}/api/players/{playerId}/presence", configuration.base_path, playerId=crate::apis::urlencode(params.player_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_beam_scope {
        req_builder = req_builder.header("X-BEAM-SCOPE", param_value.to_string());
    }
    if let Some(param_value) = params.x_beam_gamertag {
        req_builder = req_builder.header("X-BEAM-GAMERTAG", param_value.to_string());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiPlayersPlayerIdPresencePutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn api_players_player_id_presence_status_put(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdPresenceStatusPutParams) -> Result<models::OnlineStatus, Error<ApiPlayersPlayerIdPresenceStatusPutError>> {

    let uri_str = format!("{}/api/players/{playerId}/presence/status", configuration.base_path, playerId=crate::apis::urlencode(params.player_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_beam_scope {
        req_builder = req_builder.header("X-BEAM-SCOPE", param_value.to_string());
    }
    if let Some(param_value) = params.x_beam_gamertag {
        req_builder = req_builder.header("X-BEAM-GAMERTAG", param_value.to_string());
    }
    req_builder = req_builder.json(&params.set_presence_status_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OnlineStatus`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OnlineStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiPlayersPlayerIdPresenceStatusPutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

