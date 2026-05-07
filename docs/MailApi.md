# \MailApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_mail_attachments_put**](MailApi.md#basic_mail_attachments_put) | **PUT** /basic/mail/attachments | 
[**basic_mail_bulk_post**](MailApi.md#basic_mail_bulk_post) | **POST** /basic/mail/bulk | 
[**basic_mail_get**](MailApi.md#basic_mail_get) | **GET** /basic/mail/ | 
[**basic_mail_put**](MailApi.md#basic_mail_put) | **PUT** /basic/mail/ | 
[**basic_mail_template_get**](MailApi.md#basic_mail_template_get) | **GET** /basic/mail/template | 
[**object_mail_object_id_accept_many_put**](MailApi.md#object_mail_object_id_accept_many_put) | **PUT** /object/mail/{objectId}/accept/many | 
[**object_mail_object_id_bulk_post**](MailApi.md#object_mail_object_id_bulk_post) | **POST** /object/mail/{objectId}/bulk | 
[**object_mail_object_id_bulk_put**](MailApi.md#object_mail_object_id_bulk_put) | **PUT** /object/mail/{objectId}/bulk | 
[**object_mail_object_id_categories_get**](MailApi.md#object_mail_object_id_categories_get) | **GET** /object/mail/{objectId}/categories | 
[**object_mail_object_id_detail_get**](MailApi.md#object_mail_object_id_detail_get) | **GET** /object/mail/{objectId}/detail | 
[**object_mail_object_id_get**](MailApi.md#object_mail_object_id_get) | **GET** /object/mail/{objectId}/ | 
[**object_mail_object_id_post**](MailApi.md#object_mail_object_id_post) | **POST** /object/mail/{objectId}/ | 
[**object_mail_object_id_put**](MailApi.md#object_mail_object_id_put) | **PUT** /object/mail/{objectId}/ | 
[**object_mail_object_id_search_post**](MailApi.md#object_mail_object_id_search_post) | **POST** /object/mail/{objectId}/search | 



## basic_mail_attachments_put

> models::MailSuccessResponse basic_mail_attachments_put(x_beam_gamertag, accept_multiple_attachments)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**accept_multiple_attachments** | Option<[**AcceptMultipleAttachments**](AcceptMultipleAttachments.md)> |  |  |

### Return type

[**models::MailSuccessResponse**](MailSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_mail_bulk_post

> models::MailSuccessResponse basic_mail_bulk_post(x_beam_gamertag, bulk_send_mail_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**bulk_send_mail_request** | Option<[**BulkSendMailRequest**](BulkSendMailRequest.md)> |  |  |

### Return type

[**models::MailSuccessResponse**](MailSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_mail_get

> models::MailResponse basic_mail_get(mid, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mid** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::MailResponse**](MailResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_mail_put

> models::MailSuccessResponse basic_mail_put(x_beam_gamertag, update_mail_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**update_mail_request** | Option<[**UpdateMailRequest**](UpdateMailRequest.md)> |  |  |

### Return type

[**models::MailSuccessResponse**](MailSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_mail_template_get

> models::MailTemplate basic_mail_template_get(template_name, gamer_tag, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_name** | **String** |  | [required] |
**gamer_tag** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::MailTemplate**](MailTemplate.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_accept_many_put

> models::MailSuccessResponse object_mail_object_id_accept_many_put(object_id, x_beam_gamertag, accept_multiple_attachments)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**accept_multiple_attachments** | Option<[**AcceptMultipleAttachments**](AcceptMultipleAttachments.md)> |  |  |

### Return type

[**models::MailSuccessResponse**](MailSuccessResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_bulk_post

> models::MailSuccessResponse object_mail_object_id_bulk_post(object_id, x_beam_gamertag, bulk_send_mail_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**bulk_send_mail_request** | Option<[**BulkSendMailRequest**](BulkSendMailRequest.md)> |  |  |

### Return type

[**models::MailSuccessResponse**](MailSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_bulk_put

> models::MailSuccessResponse object_mail_object_id_bulk_put(object_id, x_beam_gamertag, bulk_update_mail_object_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**bulk_update_mail_object_request** | Option<[**BulkUpdateMailObjectRequest**](BulkUpdateMailObjectRequest.md)> |  |  |

### Return type

[**models::MailSuccessResponse**](MailSuccessResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_categories_get

> models::ListMailCategoriesResponse object_mail_object_id_categories_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ListMailCategoriesResponse**](ListMailCategoriesResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_detail_get

> models::MailResponse object_mail_object_id_detail_get(mid, object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mid** | **i64** |  | [required] |
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::MailResponse**](MailResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_get

> models::MailQueryResponse object_mail_object_id_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::MailQueryResponse**](MailQueryResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_post

> models::SendMailResponse object_mail_object_id_post(object_id, x_beam_gamertag, send_mail_object_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**send_mail_object_request** | Option<[**SendMailObjectRequest**](SendMailObjectRequest.md)> |  |  |

### Return type

[**models::SendMailResponse**](SendMailResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_put

> models::MailSuccessResponse object_mail_object_id_put(object_id, x_beam_gamertag, update_mail_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**update_mail_request** | Option<[**UpdateMailRequest**](UpdateMailRequest.md)> |  |  |

### Return type

[**models::MailSuccessResponse**](MailSuccessResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_mail_object_id_search_post

> models::MailSearchResponse object_mail_object_id_search_post(object_id, x_beam_gamertag, mail_search_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**mail_search_request** | Option<[**MailSearchRequest**](MailSearchRequest.md)> |  |  |

### Return type

[**models::MailSearchResponse**](MailSearchResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

