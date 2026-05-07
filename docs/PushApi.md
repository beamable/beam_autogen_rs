# \PushApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_push_register_post**](PushApi.md#basic_push_register_post) | **POST** /basic/push/register | 
[**basic_push_send_post**](PushApi.md#basic_push_send_post) | **POST** /basic/push/send | 



## basic_push_register_post

> serde_json::Value basic_push_register_post(x_beam_gamertag, register_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**register_req** | Option<[**RegisterReq**](RegisterReq.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_push_send_post

> serde_json::Value basic_push_send_post(x_beam_gamertag, send_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**send_req** | Option<[**SendReq**](SendReq.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

