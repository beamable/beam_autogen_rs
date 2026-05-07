# \PlayerStatsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_stats_delete**](PlayerStatsApi.md#api_players_player_id_stats_delete) | **DELETE** /api/players/{playerId}/stats | 
[**api_players_player_id_stats_get**](PlayerStatsApi.md#api_players_player_id_stats_get) | **GET** /api/players/{playerId}/stats | 
[**api_players_player_id_stats_post**](PlayerStatsApi.md#api_players_player_id_stats_post) | **POST** /api/players/{playerId}/stats | 



## api_players_player_id_stats_delete

> models::PlayerStatsActorCommonResponse api_players_player_id_stats_delete(player_id, x_beam_gamertag, x_beam_timeout, domain, visibility, keys)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**domain** | Option<**String**> |  |  |[default to game]
**visibility** | Option<**String**> |  |  |
**keys** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::PlayerStatsActorCommonResponse**](PlayerStatsActorCommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_stats_get

> models::GetStatsResponse api_players_player_id_stats_get(player_id, x_beam_gamertag, x_beam_timeout, domain, visibility, keys)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**domain** | Option<**String**> |  |  |[default to game]
**visibility** | Option<**String**> |  |  |
**keys** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::GetStatsResponse**](GetStatsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_stats_post

> models::PlayerStatsActorCommonResponse api_players_player_id_stats_post(player_id, x_beam_gamertag, x_beam_timeout, domain, visibility, set_stats_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**domain** | Option<**String**> |  |  |[default to game]
**visibility** | Option<**String**> |  |  |
**set_stats_request** | Option<[**SetStatsRequest**](SetStatsRequest.md)> |  |  |

### Return type

[**models::PlayerStatsActorCommonResponse**](PlayerStatsActorCommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

