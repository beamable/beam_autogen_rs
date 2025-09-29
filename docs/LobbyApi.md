# \LobbyApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_lobbies_get**](LobbyApi.md#api_lobbies_get) | **GET** /api/lobbies | 
[**api_lobbies_id_delete**](LobbyApi.md#api_lobbies_id_delete) | **DELETE** /api/lobbies/{id} | 
[**api_lobbies_id_get**](LobbyApi.md#api_lobbies_id_get) | **GET** /api/lobbies/{id} | 
[**api_lobbies_id_metadata_put**](LobbyApi.md#api_lobbies_id_metadata_put) | **PUT** /api/lobbies/{id}/metadata | 
[**api_lobbies_id_put**](LobbyApi.md#api_lobbies_id_put) | **PUT** /api/lobbies/{id} | 
[**api_lobbies_id_server_post**](LobbyApi.md#api_lobbies_id_server_post) | **POST** /api/lobbies/{id}/server | 
[**api_lobbies_id_tags_delete**](LobbyApi.md#api_lobbies_id_tags_delete) | **DELETE** /api/lobbies/{id}/tags | 
[**api_lobbies_id_tags_put**](LobbyApi.md#api_lobbies_id_tags_put) | **PUT** /api/lobbies/{id}/tags | 
[**api_lobbies_passcode_put**](LobbyApi.md#api_lobbies_passcode_put) | **PUT** /api/lobbies/passcode | 
[**api_lobbies_post**](LobbyApi.md#api_lobbies_post) | **POST** /api/lobbies | 
[**api_lobbies_put**](LobbyApi.md#api_lobbies_put) | **PUT** /api/lobbies | 



## api_lobbies_get

> models::LobbyQueryResponse api_lobbies_get(x_beam_scope, x_beam_gamertag, skip, limit, match_type)


Query for active lobbies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**skip** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**match_type** | Option<**String**> |  |  |

### Return type

[**models::LobbyQueryResponse**](LobbyQueryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_id_delete

> serde_json::Value api_lobbies_id_delete(id, x_beam_scope, x_beam_gamertag, remove_from_lobby)


Remove the requested player from the lobby. The host is able to remove anyone. Others may only remove themselves without error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the lobby | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**remove_from_lobby** | Option<[**RemoveFromLobby**](RemoveFromLobby.md)> | Request including the player requested to remove |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_id_get

> models::Lobby api_lobbies_id_get(id, x_beam_scope, x_beam_gamertag)


Get the current status of a lobby by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The lobby id. | [required] |
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


## api_lobbies_id_metadata_put

> models::Lobby api_lobbies_id_metadata_put(id, x_beam_scope, x_beam_gamertag, update_lobby)


Update the properties of a lobby

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the lobby | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**update_lobby** | Option<[**UpdateLobby**](UpdateLobby.md)> | The update lobby request. |  |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_id_put

> models::Lobby api_lobbies_id_put(id, x_beam_scope, x_beam_gamertag, join_lobby)


Join a lobby

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the lobby | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**join_lobby** | Option<[**JoinLobby**](JoinLobby.md)> | The join lobby request. Includes tags. |  |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_id_server_post

> serde_json::Value api_lobbies_id_server_post(id, x_beam_scope, x_beam_gamertag, create_federated_game_server)


Invoke the Lobby actor to make the federated game server request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the lobby | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**create_federated_game_server** | Option<[**CreateFederatedGameServer**](CreateFederatedGameServer.md)> | Includes an optional matchtype |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_id_tags_delete

> models::Lobby api_lobbies_id_tags_delete(id, x_beam_scope, x_beam_gamertag, remove_tags)


Remove the request tags from the requested player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the lobby | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**remove_tags** | Option<[**RemoveTags**](RemoveTags.md)> | Includes the player ID and the tags to remove. |  |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_id_tags_put

> models::Lobby api_lobbies_id_tags_put(id, x_beam_scope, x_beam_gamertag, add_tags)


Add the request tags to the requested player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the lobby | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**add_tags** | Option<[**AddTags**](AddTags.md)> | Includes the player ID and tags to add. |  |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_passcode_put

> models::Lobby api_lobbies_passcode_put(x_beam_scope, x_beam_gamertag, join_lobby)


Join a lobby by passcode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**join_lobby** | Option<[**JoinLobby**](JoinLobby.md)> | The join lobby request. Includes tags. |  |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_post

> models::Lobby api_lobbies_post(x_beam_scope, x_beam_gamertag, create_lobby)


Create a lobby. A leader is not necessary to create a lobby.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**create_lobby** | Option<[**CreateLobby**](CreateLobby.md)> | The Create request. |  |

### Return type

[**models::Lobby**](Lobby.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_lobbies_put

> models::SetLobbyResponse api_lobbies_put(x_beam_scope, x_beam_gamertag, lobby)


Exposes the internal \"SetLobby\" behavior as an Admin only endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**lobby** | Option<[**Lobby**](Lobby.md)> |  |  |

### Return type

[**models::SetLobbyResponse**](SetLobbyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

