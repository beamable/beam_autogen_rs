# \PresenceApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_presence_query_post**](PresenceApi.md#api_presence_query_post) | **POST** /api/presence/query | 



## api_presence_query_post

> models::PlayersStatusResponse api_presence_query_post(x_beam_gamertag, x_beam_timeout, online_status_query)


Query the online status for a batch of players.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**online_status_query** | Option<[**OnlineStatusQuery**](OnlineStatusQuery.md)> | Query containing the player IDs to look up. |  |

### Return type

[**models::PlayersStatusResponse**](PlayersStatusResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

