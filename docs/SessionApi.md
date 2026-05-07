# \SessionApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_sessions_heartbeat_post**](SessionApi.md#api_sessions_heartbeat_post) | **POST** /api/sessions/heartbeat | 
[**api_sessions_post**](SessionApi.md#api_sessions_post) | **POST** /api/sessions | 
[**api_sessions_status_get**](SessionApi.md#api_sessions_status_get) | **GET** /api/sessions/status | 
[**basic_session_client_history_get**](SessionApi.md#basic_session_client_history_get) | **GET** /basic/session/client/history | 
[**basic_session_heartbeat_post**](SessionApi.md#basic_session_heartbeat_post) | **POST** /basic/session/heartbeat | 
[**basic_session_history_get**](SessionApi.md#basic_session_history_get) | **GET** /basic/session/history | 
[**basic_session_post**](SessionApi.md#basic_session_post) | **POST** /basic/session/ | 
[**basic_session_status_get**](SessionApi.md#basic_session_status_get) | **GET** /basic/session/status | 



## api_sessions_heartbeat_post

> serde_json::Value api_sessions_heartbeat_post(x_beam_gamertag, x_beam_timeout)


Legacy session heartbeat. Bridges into the actor presence system so old clients that don't use WebSocket presence pings appear online.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_sessions_post

> serde_json::Value api_sessions_post(x_beam_gamertag, x_beam_timeout, session_actor_start_session_request)


Start a session for a player. Used by old clients whose POST /basic/session is forwarded by the Scala proxy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**session_actor_start_session_request** | Option<[**SessionActorStartSessionRequest**](SessionActorStartSessionRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_sessions_status_get

> serde_json::Value api_sessions_status_get(x_beam_gamertag, x_beam_timeout, player_ids, interval_secs)


Legacy online status endpoint. Handles GET /sessions/status forwarded by the Scala proxy for old clients that query online status via the session service rather than the presence API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**player_ids** | Option<**String**> |  |  |
**interval_secs** | Option<**i64**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_session_client_history_get

> models::SessionBasicSessionClientHistoryResponse basic_session_client_history_get(x_beam_gamertag, month, year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**month** | Option<**i32**> |  |  |
**year** | Option<**i32**> |  |  |

### Return type

[**models::SessionBasicSessionClientHistoryResponse**](SessionBasicSessionClientHistoryResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_session_heartbeat_post

> models::SessionHeartbeat basic_session_heartbeat_post(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::SessionHeartbeat**](SessionHeartbeat.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_session_history_get

> models::SessionBasicSessionHistoryResponse basic_session_history_get(dbid, x_beam_gamertag, month, year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbid** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**month** | Option<**i32**> |  |  |
**year** | Option<**i32**> |  |  |

### Return type

[**models::SessionBasicSessionHistoryResponse**](SessionBasicSessionHistoryResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_session_post

> models::StartSessionResponse basic_session_post(x_beam_gamertag, session_basic_start_session_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**session_basic_start_session_request** | Option<[**SessionBasicStartSessionRequest**](SessionBasicStartSessionRequest.md)> |  |  |

### Return type

[**models::StartSessionResponse**](StartSessionResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_session_status_get

> models::OnlineStatusResponses basic_session_status_get(player_ids, interval_secs, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_ids** | **String** |  | [required] |
**interval_secs** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::OnlineStatusResponses**](OnlineStatusResponses.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

