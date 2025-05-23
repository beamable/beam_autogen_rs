# \PlayerPartyApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_parties_delete**](PlayerPartyApi.md#api_players_player_id_parties_delete) | **DELETE** /api/players/{playerId}/parties | 
[**api_players_player_id_parties_get**](PlayerPartyApi.md#api_players_player_id_parties_get) | **GET** /api/players/{playerId}/parties | 
[**api_players_player_id_parties_invites_get**](PlayerPartyApi.md#api_players_player_id_parties_invites_get) | **GET** /api/players/{playerId}/parties/invites | 
[**api_players_player_id_party_invites_get**](PlayerPartyApi.md#api_players_player_id_party_invites_get) | **GET** /api/players/{playerId}/party/invites | 



## api_players_player_id_parties_delete

> serde_json::Value api_players_player_id_parties_delete(player_id, x_beam_scope, x_beam_gamertag)


If the requested player is in a party, remove the player

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


## api_players_player_id_parties_get

> models::Party api_players_player_id_parties_get(player_id, x_beam_scope, x_beam_gamertag)


Fetch the requested player's party information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_parties_invites_get

> models::PartyInvitesForPlayerResponse api_players_player_id_parties_invites_get(player_id, x_beam_scope, x_beam_gamertag)


Return list of party invites for player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | PlayerId | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::PartyInvitesForPlayerResponse**](PartyInvitesForPlayerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_party_invites_get

> models::PartyInvitesForPlayerResponse api_players_player_id_party_invites_get(player_id, x_beam_scope, x_beam_gamertag)


Return list of party invites for player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | PlayerId | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::PartyInvitesForPlayerResponse**](PartyInvitesForPlayerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

