# \PlayerLobbyApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_lobbies_delete**](PlayerLobbyApi.md#api_players_player_id_lobbies_delete) | **DELETE** /api/players/{playerId}/lobbies | 
[**api_players_player_id_lobbies_get**](PlayerLobbyApi.md#api_players_player_id_lobbies_get) | **GET** /api/players/{playerId}/lobbies | 



## api_players_player_id_lobbies_delete

> serde_json::Value api_players_player_id_lobbies_delete(player_id, x_beam_gamertag, x_beam_timeout)


If the requested player is in a lobby, remove the player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |
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


## api_players_player_id_lobbies_get

> models::Lobby api_players_player_id_lobbies_get(player_id, x_beam_gamertag, x_beam_timeout)


Fetch the requested player's lobby information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

