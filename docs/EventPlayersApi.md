# \EventPlayersApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**object_event_players_object_id_claim_post**](EventPlayersApi.md#object_event_players_object_id_claim_post) | **POST** /object/event-players/{objectId}/claim | 
[**object_event_players_object_id_get**](EventPlayersApi.md#object_event_players_object_id_get) | **GET** /object/event-players/{objectId}/ | 
[**object_event_players_object_id_score_put**](EventPlayersApi.md#object_event_players_object_id_score_put) | **PUT** /object/event-players/{objectId}/score | 



## object_event_players_object_id_claim_post

> models::EventClaimResponse object_event_players_object_id_claim_post(object_id, x_beam_gamertag, event_claim_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**event_claim_request** | Option<[**EventClaimRequest**](EventClaimRequest.md)> |  |  |

### Return type

[**models::EventClaimResponse**](EventClaimResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_event_players_object_id_get

> models::EventPlayerView object_event_players_object_id_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::EventPlayerView**](EventPlayerView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_event_players_object_id_score_put

> models::CommonResponse object_event_players_object_id_score_put(object_id, x_beam_gamertag, event_score_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**event_score_request** | Option<[**EventScoreRequest**](EventScoreRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

