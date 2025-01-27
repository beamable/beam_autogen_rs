# \PresenceApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_presence_query_post**](PresenceApi.md#api_presence_query_post) | **POST** /api/presence/query | 



## api_presence_query_post

> models::PlayersStatusResponse api_presence_query_post(x_beam_scope, x_beam_gamertag, online_status_query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**online_status_query** | Option<[**OnlineStatusQuery**](OnlineStatusQuery.md)> |  |  |

### Return type

[**models::PlayersStatusResponse**](PlayersStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

