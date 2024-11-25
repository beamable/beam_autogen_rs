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
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`api_players_player_id_lobbies_delete`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdLobbiesDeleteParams {
    /// Player Id
    pub player_id: String
}

/// struct for passing parameters to the method [`api_players_player_id_lobbies_get`]
#[derive(Clone, Debug)]
pub struct ApiPlayersPlayerIdLobbiesGetParams {
    /// Player Id
    pub player_id: String
}


/// struct for typed errors of method [`api_players_player_id_lobbies_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdLobbiesDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_players_player_id_lobbies_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPlayersPlayerIdLobbiesGetError {
    UnknownValue(serde_json::Value),
}


/// If the requested player is in a lobby, remove the player
pub async fn api_players_player_id_lobbies_delete(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdLobbiesDeleteParams) -> Result<serde_json::Value, Error<ApiPlayersPlayerIdLobbiesDeleteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let player_id = params.player_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/players/{playerId}/lobbies", local_var_configuration.base_path, playerId=crate::apis::urlencode(player_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApiPlayersPlayerIdLobbiesDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch the requested player's lobby information
pub async fn api_players_player_id_lobbies_get(configuration: &configuration::Configuration, params: ApiPlayersPlayerIdLobbiesGetParams) -> Result<models::Lobby, Error<ApiPlayersPlayerIdLobbiesGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let player_id = params.player_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/players/{playerId}/lobbies", local_var_configuration.base_path, playerId=crate::apis::urlencode(player_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApiPlayersPlayerIdLobbiesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

