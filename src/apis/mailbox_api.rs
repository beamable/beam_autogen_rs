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

/// struct for passing parameters to the method [`api_mailbox_publish_post`]
#[derive(Clone, Debug)]
pub struct ApiMailboxPublishPostParams {
    /// Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token
    pub x_beam_scope: Option<String>,
    /// Override the playerId of the requester. This is only necessary when not using a JWT bearer token.
    pub x_beam_gamertag: Option<String>,
    pub message_request: Option<models::MessageRequest>
}


/// struct for typed errors of method [`api_mailbox_publish_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiMailboxPublishPostError {
    UnknownValue(serde_json::Value),
}


pub async fn api_mailbox_publish_post(configuration: &configuration::Configuration, params: ApiMailboxPublishPostParams) -> Result<serde_json::Value, Error<ApiMailboxPublishPostError>> {

    let uri_str = format!("{}/api/mailbox/publish", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_beam_scope {
        req_builder = req_builder.header("X-BEAM-SCOPE", param_value.to_string());
    }
    if let Some(param_value) = params.x_beam_gamertag {
        req_builder = req_builder.header("X-BEAM-GAMERTAG", param_value.to_string());
    }
    req_builder = req_builder.json(&params.message_request);

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
        let entity: Option<ApiMailboxPublishPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

