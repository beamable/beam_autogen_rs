# \PlayerLobbyApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_lobbies_delete**](PlayerLobbyApi.md#api_players_player_id_lobbies_delete) | **DELETE** /api/players/{playerId}/lobbies | 
[**api_players_player_id_lobbies_get**](PlayerLobbyApi.md#api_players_player_id_lobbies_get) | **GET** /api/players/{playerId}/lobbies | 



## api_players_player_id_lobbies_delete

> serde_json::Value api_players_player_id_lobbies_delete(player_id, x_beam_scope, x_beam_gamertag)


If the requested player is in a lobby, remove the player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_lobbies_get

> models::Lobby api_players_player_id_lobbies_get(player_id, x_beam_scope, x_beam_gamertag)


Fetch the requested player's lobby information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

