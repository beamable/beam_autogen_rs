# \PlayerSessionApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_sessions_client_get**](PlayerSessionApi.md#api_players_player_id_sessions_client_get) | **GET** /api/players/{playerId}/sessions/client | 
[**api_players_player_id_sessions_get**](PlayerSessionApi.md#api_players_player_id_sessions_get) | **GET** /api/players/{playerId}/sessions | 



## api_players_player_id_sessions_client_get

> models::PlayerSessionActorSessionClientHistoryResponse api_players_player_id_sessions_client_get(player_id, x_beam_gamertag, x_beam_timeout, month, year)


Get session history for the requesting player (client endpoint).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**month** | Option<**i32**> |  |  |
**year** | Option<**i32**> |  |  |

### Return type

[**models::PlayerSessionActorSessionClientHistoryResponse**](PlayerSessionActorSessionClientHistoryResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_sessions_get

> models::PlayerSessionActorSessionHistoryResponse api_players_player_id_sessions_get(player_id, x_beam_gamertag, x_beam_timeout, month, year)


Get session history for a player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**month** | Option<**i32**> |  |  |
**year** | Option<**i32**> |  |  |

### Return type

[**models::PlayerSessionActorSessionHistoryResponse**](PlayerSessionActorSessionHistoryResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

