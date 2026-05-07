# \MailboxApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_mailbox_publish_post**](MailboxApi.md#api_mailbox_publish_post) | **POST** /api/mailbox/publish | 



## api_mailbox_publish_post

> serde_json::Value api_mailbox_publish_post(x_beam_gamertag, x_beam_timeout, message_request)


Publish a message to a player mailbox, channel mailbox, or realm broadcast.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**message_request** | Option<[**MessageRequest**](MessageRequest.md)> | Message request. Use PlayerId for player mailbox, Channel for channel mailbox, or RealmId for realm broadcast. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

