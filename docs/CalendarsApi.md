# \CalendarsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**object_calendars_object_id_claim_post**](CalendarsApi.md#object_calendars_object_id_claim_post) | **POST** /object/calendars/{objectId}/claim | 
[**object_calendars_object_id_get**](CalendarsApi.md#object_calendars_object_id_get) | **GET** /object/calendars/{objectId}/ | 



## object_calendars_object_id_claim_post

> models::CommonResponse object_calendars_object_id_claim_post(object_id, x_beam_gamertag, calendar_claim_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**calendar_claim_request** | Option<[**CalendarClaimRequest**](CalendarClaimRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_calendars_object_id_get

> models::CalendarQueryResponse object_calendars_object_id_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CalendarQueryResponse**](CalendarQueryResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

