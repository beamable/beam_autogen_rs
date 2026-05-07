# \AnnouncementsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_announcements_content_get**](AnnouncementsApi.md#basic_announcements_content_get) | **GET** /basic/announcements/content | 
[**basic_announcements_delete**](AnnouncementsApi.md#basic_announcements_delete) | **DELETE** /basic/announcements/ | 
[**basic_announcements_list_definitions_get**](AnnouncementsApi.md#basic_announcements_list_definitions_get) | **GET** /basic/announcements/list/definitions | 
[**basic_announcements_list_get**](AnnouncementsApi.md#basic_announcements_list_get) | **GET** /basic/announcements/list | 
[**basic_announcements_list_tags_get**](AnnouncementsApi.md#basic_announcements_list_tags_get) | **GET** /basic/announcements/list/tags | 
[**basic_announcements_post**](AnnouncementsApi.md#basic_announcements_post) | **POST** /basic/announcements/ | 
[**basic_announcements_search_get**](AnnouncementsApi.md#basic_announcements_search_get) | **GET** /basic/announcements/search | 
[**object_announcements_object_id_claim_post**](AnnouncementsApi.md#object_announcements_object_id_claim_post) | **POST** /object/announcements/{objectId}/claim | 
[**object_announcements_object_id_delete**](AnnouncementsApi.md#object_announcements_object_id_delete) | **DELETE** /object/announcements/{objectId}/ | 
[**object_announcements_object_id_get**](AnnouncementsApi.md#object_announcements_object_id_get) | **GET** /object/announcements/{objectId}/ | 
[**object_announcements_object_id_raw_get**](AnnouncementsApi.md#object_announcements_object_id_raw_get) | **GET** /object/announcements/{objectId}/raw | 
[**object_announcements_object_id_read_put**](AnnouncementsApi.md#object_announcements_object_id_read_put) | **PUT** /object/announcements/{objectId}/read | 



## basic_announcements_content_get

> models::AnnouncementContentResponse basic_announcements_content_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AnnouncementContentResponse**](AnnouncementContentResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_announcements_delete

> models::EmptyResponse basic_announcements_delete(x_beam_gamertag, delete_announcement_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_announcement_request** | Option<[**DeleteAnnouncementRequest**](DeleteAnnouncementRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_announcements_list_definitions_get

> models::ListDefinitionsResponse basic_announcements_list_definitions_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ListDefinitionsResponse**](ListDefinitionsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_announcements_list_get

> models::AnnouncementContentResponse basic_announcements_list_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AnnouncementContentResponse**](AnnouncementContentResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_announcements_list_tags_get

> models::ListTagsResponse basic_announcements_list_tags_get(x_beam_gamertag, tag_name_filter)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**tag_name_filter** | Option<**String**> |  |  |

### Return type

[**models::ListTagsResponse**](ListTagsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_announcements_post

> models::EmptyResponse basic_announcements_post(x_beam_gamertag, announcement_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**announcement_dto** | Option<[**AnnouncementDto**](AnnouncementDto.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_announcements_search_get

> models::AnnouncementContentResponse basic_announcements_search_get(x_beam_gamertag, date)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**date** | Option<**String**> |  |  |

### Return type

[**models::AnnouncementContentResponse**](AnnouncementContentResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_announcements_object_id_claim_post

> models::CommonResponse object_announcements_object_id_claim_post(object_id, x_beam_gamertag, announcement_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**announcement_request** | Option<[**AnnouncementRequest**](AnnouncementRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_announcements_object_id_delete

> models::CommonResponse object_announcements_object_id_delete(object_id, x_beam_gamertag, announcement_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**announcement_request** | Option<[**AnnouncementRequest**](AnnouncementRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_announcements_object_id_get

> models::AnnouncementQueryResponse object_announcements_object_id_get(object_id, x_beam_gamertag, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**include_deleted** | Option<**bool**> |  |  |

### Return type

[**models::AnnouncementQueryResponse**](AnnouncementQueryResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_announcements_object_id_raw_get

> models::AnnouncementRawResponse object_announcements_object_id_raw_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AnnouncementRawResponse**](AnnouncementRawResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_announcements_object_id_read_put

> models::CommonResponse object_announcements_object_id_read_put(object_id, x_beam_gamertag, announcement_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**announcement_request** | Option<[**AnnouncementRequest**](AnnouncementRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

