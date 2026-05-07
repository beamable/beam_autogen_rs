# \PlayerPresenceApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_presence_get**](PlayerPresenceApi.md#api_players_player_id_presence_get) | **GET** /api/players/{playerId}/presence | 
[**api_players_player_id_presence_put**](PlayerPresenceApi.md#api_players_player_id_presence_put) | **PUT** /api/players/{playerId}/presence | 
[**api_players_player_id_presence_status_put**](PlayerPresenceApi.md#api_players_player_id_presence_status_put) | **PUT** /api/players/{playerId}/presence/status | 



## api_players_player_id_presence_get

> models::OnlineStatus api_players_player_id_presence_get(player_id, x_beam_gamertag, x_beam_timeout)


Get the current online status of a player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player ID to retrieve online status for. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::OnlineStatus**](OnlineStatus.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_presence_put

> serde_json::Value api_players_player_id_presence_put(player_id, x_beam_gamertag, x_beam_timeout)


Submit a heartbeat to mark the requesting player as online.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | The player ID to heartbeat. Must match the authenticated player. | [required] |
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


## api_players_player_id_presence_status_put

> models::OnlineStatus api_players_player_id_presence_status_put(player_id, x_beam_gamertag, x_beam_timeout, set_presence_status_request)


Set a custom presence status for the requesting player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | The player ID to update. Must match the authenticated player. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**set_presence_status_request** | Option<[**SetPresenceStatusRequest**](SetPresenceStatusRequest.md)> | Presence status to set. |  |

### Return type

[**models::OnlineStatus**](OnlineStatus.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

