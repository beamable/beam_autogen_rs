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

/// struct for passing parameters to the method [`api_players_player_id_parties_delete`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdPartiesDeleteParams {
    /// Player Id
    pub player_id: String,
    /// Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token
    pub x_beam_scope: Option<String>,
    /// Override the playerId of the requester. This is only necessary when not using a JWT bearer token.
    pub x_beam_gamertag: Option<String>
}

/// struct for passing parameters to the method [`api_players_player_id_parties_get`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdPartiesGetParams {
    /// Player Id
    pub player_id: String,
    /// Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token
    pub x_beam_scope: Option<String>,
    /// Override the playerId of the requester. This is only necessary when not using a JWT bearer token.
    pub x_beam_gamertag: Option<String>
}

/// struct for passing parameters to the method [`api_players_player_id_parties_invites_get`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdPartiesInvitesGetParams {
    /// PlayerId
    pub player_id: String,
    /// Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token
    pub x_beam_scope: Option<String>,
    /// Override the playerId of the requester. This is only necessary when not using a JWT bearer token.
    pub x_beam_gamertag: Option<String>
}

/// struct for passing parameters to the method [`api_players_player_id_party_invites_get`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdPartyInvitesGetParams {
    /// PlayerId
    pub player_id: String,
    /// Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token
    pub x_beam_scope: Option<String>,
    /// Override the playerId of the requester. This is only necessary when not using a JWT bearer token.
    pub x_beam_gamertag: Option<String>
}


/// struct for typed errors of method [`api_players_player_id_parties_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdPartiesDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_players_player_id_parties_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdPartiesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_players_player_id_parties_invites_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdPartiesInvitesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_players_player_id_party_invites_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdPartyInvitesGetError {
    UnknownValue(serde_json::Value),
}


/// If the requested player is in a party, remove the player
pub async fn api_players_player_id_parties_delete(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdPartiesDeleteParams) -> Result<serde_json::Value, Error<ApiPlayersPlayerIdPartiesDeleteError>> {

    let uri_str = format!("{}/api/players/{playerId}/parties", configuration.base_path, playerId=crate::apis::urlencode(params.player_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

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
        let entity: Option<ApiPlayersPlayerIdPartiesDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Fetch the requested player's party information
pub async fn api_players_player_id_parties_get(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdPartiesGetParams) -> Result<models::Party, Error<ApiPlayersPlayerIdPartiesGetError>> {

    let uri_str = format!("{}/api/players/{playerId}/parties", configuration.base_path, playerId=crate::apis::urlencode(params.player_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Party`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Party`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiPlayersPlayerIdPartiesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return list of party invites for player.
pub async fn api_players_player_id_parties_invites_get(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdPartiesInvitesGetParams) -> Result<models::PartyInvitesForPlayerResponse, Error<ApiPlayersPlayerIdPartiesInvitesGetError>> {

    let uri_str = format!("{}/api/players/{playerId}/parties/invites", configuration.base_path, playerId=crate::apis::urlencode(params.player_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PartyInvitesForPlayerResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PartyInvitesForPlayerResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiPlayersPlayerIdPartiesInvitesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return list of party invites for player.
pub async fn api_players_player_id_party_invites_get(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdPartyInvitesGetParams) -> Result<models::PartyInvitesForPlayerResponse, Error<ApiPlayersPlayerIdPartyInvitesGetError>> {

    let uri_str = format!("{}/api/players/{playerId}/party/invites", configuration.base_path, playerId=crate::apis::urlencode(params.player_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PartyInvitesForPlayerResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PartyInvitesForPlayerResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiPlayersPlayerIdPartyInvitesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

