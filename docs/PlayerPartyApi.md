# \PlayerPartyApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_parties_delete**](PlayerPartyApi.md#api_players_player_id_parties_delete) | **DELETE** /api/players/{playerId}/parties | 
[**api_players_player_id_parties_get**](PlayerPartyApi.md#api_players_player_id_parties_get) | **GET** /api/players/{playerId}/parties | 
[**api_players_player_id_parties_invites_get**](PlayerPartyApi.md#api_players_player_id_parties_invites_get) | **GET** /api/players/{playerId}/parties/invites | 
[**api_players_player_id_party_invites_get**](PlayerPartyApi.md#api_players_player_id_party_invites_get) | **GET** /api/players/{playerId}/party/invites | 



## api_players_player_id_parties_delete

> serde_json::Value api_players_player_id_parties_delete(player_id, x_beam_gamertag, x_beam_timeout)


If the requested player is in a party, remove the player

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


## api_players_player_id_parties_get

> models::Party api_players_player_id_parties_get(player_id, x_beam_gamertag, x_beam_timeout)


Fetch the requested player's party information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_parties_invites_get

> models::PartyInvitesForPlayerResponse api_players_player_id_parties_invites_get(player_id, x_beam_gamertag, x_beam_timeout)


Return list of party invites for player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | PlayerId | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::PartyInvitesForPlayerResponse**](PartyInvitesForPlayerResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_party_invites_get

> models::PartyInvitesForPlayerResponse api_players_player_id_party_invites_get(player_id, x_beam_gamertag, x_beam_timeout)


Return list of party invites for player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | PlayerId | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::PartyInvitesForPlayerResponse**](PartyInvitesForPlayerResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

