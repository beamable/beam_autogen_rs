# \ChatV2Api

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**object_chat_v2_object_id_get**](ChatV2Api.md#object_chat_v2_object_id_get) | **GET** /object/chatV2/{objectId}/ | 
[**object_chat_v2_object_id_messages_post**](ChatV2Api.md#object_chat_v2_object_id_messages_post) | **POST** /object/chatV2/{objectId}/messages | 
[**object_chat_v2_object_id_rooms_delete**](ChatV2Api.md#object_chat_v2_object_id_rooms_delete) | **DELETE** /object/chatV2/{objectId}/rooms | 
[**object_chat_v2_object_id_rooms_get**](ChatV2Api.md#object_chat_v2_object_id_rooms_get) | **GET** /object/chatV2/{objectId}/rooms | 
[**object_chat_v2_object_id_rooms_post**](ChatV2Api.md#object_chat_v2_object_id_rooms_post) | **POST** /object/chatV2/{objectId}/rooms | 



## object_chat_v2_object_id_get

> models::GetRoomsResponse object_chat_v2_object_id_get(object_id, x_beam_gamertag, scope)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**scope** | Option<**String**> |  |  |

### Return type

[**models::GetRoomsResponse**](GetRoomsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_chat_v2_object_id_messages_post

> models::SendMessageResponse object_chat_v2_object_id_messages_post(object_id, x_beam_gamertag, send_message_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**send_message_request** | Option<[**SendMessageRequest**](SendMessageRequest.md)> |  |  |

### Return type

[**models::SendMessageResponse**](SendMessageResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_chat_v2_object_id_rooms_delete

> models::LeaveRoomResponse object_chat_v2_object_id_rooms_delete(object_id, x_beam_gamertag, leave_room_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**leave_room_request** | Option<[**LeaveRoomRequest**](LeaveRoomRequest.md)> |  |  |

### Return type

[**models::LeaveRoomResponse**](LeaveRoomResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_chat_v2_object_id_rooms_get

> models::GetRoomsResponse object_chat_v2_object_id_rooms_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetRoomsResponse**](GetRoomsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_chat_v2_object_id_rooms_post

> models::CreateRoomResponse object_chat_v2_object_id_rooms_post(object_id, x_beam_gamertag, create_room_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_room_request** | Option<[**CreateRoomRequest**](CreateRoomRequest.md)> |  |  |

### Return type

[**models::CreateRoomResponse**](CreateRoomResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

