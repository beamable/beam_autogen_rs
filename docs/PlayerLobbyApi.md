# \PlayerLobbyApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_lobbies_delete**](PlayerLobbyApi.md#api_players_player_id_lobbies_delete) | **DELETE** /api/players/{playerId}/lobbies | 
[**api_players_player_id_lobbies_get**](PlayerLobbyApi.md#api_players_player_id_lobbies_get) | **GET** /api/players/{playerId}/lobbies | 



## api_players_player_id_lobbies_delete

> serde_json::Value api_players_player_id_lobbies_delete(player_id)


If the requested player is in a lobby, remove the player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_lobbies_get

> models::Lobby api_players_player_id_lobbies_get(player_id)


Fetch the requested player's lobby information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

