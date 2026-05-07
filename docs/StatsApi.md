# \StatsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_stats_batch_post**](StatsApi.md#api_stats_batch_post) | **POST** /api/stats/batch | 
[**api_stats_batch_put**](StatsApi.md#api_stats_batch_put) | **PUT** /api/stats/batch | 
[**api_stats_item_type_id_delete**](StatsApi.md#api_stats_item_type_id_delete) | **DELETE** /api/stats/{itemType}/{id} | 
[**api_stats_item_type_id_get**](StatsApi.md#api_stats_item_type_id_get) | **GET** /api/stats/{itemType}/{id} | 
[**api_stats_item_type_id_post**](StatsApi.md#api_stats_item_type_id_post) | **POST** /api/stats/{itemType}/{id} | 
[**api_stats_query_extended_post**](StatsApi.md#api_stats_query_extended_post) | **POST** /api/stats/query/extended | 
[**api_stats_query_post**](StatsApi.md#api_stats_query_post) | **POST** /api/stats/query | 
[**api_stats_subscription_delete**](StatsApi.md#api_stats_subscription_delete) | **DELETE** /api/stats/subscription | 
[**api_stats_subscription_put**](StatsApi.md#api_stats_subscription_put) | **PUT** /api/stats/subscription | 
[**basic_stats_batch_post**](StatsApi.md#basic_stats_batch_post) | **POST** /basic/stats/batch | 
[**basic_stats_client_batch_get**](StatsApi.md#basic_stats_client_batch_get) | **GET** /basic/stats/client/batch | 
[**basic_stats_search_extended_post**](StatsApi.md#basic_stats_search_extended_post) | **POST** /basic/stats/search/extended | 
[**basic_stats_search_post**](StatsApi.md#basic_stats_search_post) | **POST** /basic/stats/search | 
[**basic_stats_subscribe_delete**](StatsApi.md#basic_stats_subscribe_delete) | **DELETE** /basic/stats/subscribe | 
[**basic_stats_subscribe_put**](StatsApi.md#basic_stats_subscribe_put) | **PUT** /basic/stats/subscribe | 
[**object_stats_object_id_client_get**](StatsApi.md#object_stats_object_id_client_get) | **GET** /object/stats/{objectId}/client | 
[**object_stats_object_id_client_post**](StatsApi.md#object_stats_object_id_client_post) | **POST** /object/stats/{objectId}/client | 
[**object_stats_object_id_client_stringlist_post**](StatsApi.md#object_stats_object_id_client_stringlist_post) | **POST** /object/stats/{objectId}/client/stringlist | 
[**object_stats_object_id_delete**](StatsApi.md#object_stats_object_id_delete) | **DELETE** /object/stats/{objectId}/ | 
[**object_stats_object_id_get**](StatsApi.md#object_stats_object_id_get) | **GET** /object/stats/{objectId}/ | 
[**object_stats_object_id_post**](StatsApi.md#object_stats_object_id_post) | **POST** /object/stats/{objectId}/ | 



## api_stats_batch_post

> models::BatchGetStatsResponse api_stats_batch_post(x_beam_gamertag, x_beam_timeout, batch_get_request)


Batch get stats for multiple targets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**batch_get_request** | Option<[**BatchGetRequest**](BatchGetRequest.md)> |  |  |

### Return type

[**models::BatchGetStatsResponse**](BatchGetStatsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stats_batch_put

> models::StatsActorCommonResponse api_stats_batch_put(x_beam_gamertag, x_beam_timeout, batch_write_request)


Batch write stats for multiple targets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**batch_write_request** | Option<[**BatchWriteRequest**](BatchWriteRequest.md)> |  |  |

### Return type

[**models::StatsActorCommonResponse**](StatsActorCommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stats_item_type_id_delete

> models::StatsActorCommonResponse api_stats_item_type_id_delete(item_type, id, x_beam_gamertag, x_beam_timeout, domain, visibility, keys)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_type** | **String** |  | [required] |
**id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**domain** | Option<**String**> |  |  |
**visibility** | Option<[**StatsVisibility**](StatsVisibility.md)> |  |  |
**keys** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::StatsActorCommonResponse**](StatsActorCommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stats_item_type_id_get

> models::GetStatsResponse api_stats_item_type_id_get(item_type, id, x_beam_gamertag, x_beam_timeout, domain, visibility, keys)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_type** | **String** |  | [required] |
**id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**domain** | Option<**String**> |  |  |[default to game]
**visibility** | Option<[**StatsVisibility**](StatsVisibility.md)> |  |  |
**keys** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::GetStatsResponse**](GetStatsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stats_item_type_id_post

> models::StatsActorCommonResponse api_stats_item_type_id_post(item_type, id, x_beam_gamertag, x_beam_timeout, domain, visibility, set_stats_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_type** | **String** |  | [required] |
**id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**domain** | Option<**String**> |  |  |
**visibility** | Option<[**StatsVisibility**](StatsVisibility.md)> |  |  |
**set_stats_request** | Option<[**SetStatsRequest**](SetStatsRequest.md)> |  |  |

### Return type

[**models::StatsActorCommonResponse**](StatsActorCommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stats_query_extended_post

> models::StatsSearchExtendedResponse api_stats_query_extended_post(x_beam_gamertag, x_beam_timeout, stats_search_extended_request)


Query for objects by stat criteria. Returns matching object IDs with their stat values, optionally filtered by statKeys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**stats_search_extended_request** | Option<[**StatsSearchExtendedRequest**](StatsSearchExtendedRequest.md)> |  |  |

### Return type

[**models::StatsSearchExtendedResponse**](StatsSearchExtendedResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stats_query_post

> models::StatsActorStatsSearchResponse api_stats_query_post(x_beam_gamertag, x_beam_timeout, stats_actor_stats_search_request)


Query for objects by stat criteria. Returns matching object IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**stats_actor_stats_search_request** | Option<[**StatsActorStatsSearchRequest**](StatsActorStatsSearchRequest.md)> |  |  |

### Return type

[**models::StatsActorStatsSearchResponse**](StatsActorStatsSearchResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stats_subscription_delete

> models::StatsActorCommonResponse api_stats_subscription_delete(x_beam_gamertag, x_beam_timeout, stats_unsubscribe_request)


Unsubscribe a service from stat change notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**stats_unsubscribe_request** | Option<[**StatsUnsubscribeRequest**](StatsUnsubscribeRequest.md)> |  |  |

### Return type

[**models::StatsActorCommonResponse**](StatsActorCommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stats_subscription_put

> models::StatsActorCommonResponse api_stats_subscription_put(x_beam_gamertag, x_beam_timeout, stats_subscribe_request)


Subscribe a service to stat change notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**stats_subscribe_request** | Option<[**StatsSubscribeRequest**](StatsSubscribeRequest.md)> |  |  |

### Return type

[**models::StatsActorCommonResponse**](StatsActorCommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_stats_batch_post

> models::EmptyResponse basic_stats_batch_post(x_beam_gamertag, batch_set_stats_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**batch_set_stats_request** | Option<[**BatchSetStatsRequest**](BatchSetStatsRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_stats_client_batch_get

> models::BatchReadStatsResponse basic_stats_client_batch_get(object_ids, x_beam_gamertag, stats, format)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_ids** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stats** | Option<**String**> |  |  |
**format** | Option<**String**> |  |  |

### Return type

[**models::BatchReadStatsResponse**](BatchReadStatsResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_stats_search_extended_post

> models::SearchExtendedResponse basic_stats_search_extended_post(x_beam_gamertag, search_extended_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**search_extended_request** | Option<[**SearchExtendedRequest**](SearchExtendedRequest.md)> |  |  |

### Return type

[**models::SearchExtendedResponse**](SearchExtendedResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_stats_search_post

> models::StatsBasicStatsSearchResponse basic_stats_search_post(x_beam_gamertag, stats_basic_stats_search_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stats_basic_stats_search_request** | Option<[**StatsBasicStatsSearchRequest**](StatsBasicStatsSearchRequest.md)> |  |  |

### Return type

[**models::StatsBasicStatsSearchResponse**](StatsBasicStatsSearchResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_stats_subscribe_delete

> models::CommonResponse basic_stats_subscribe_delete(x_beam_gamertag, stats_unsubscribe_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stats_unsubscribe_request** | Option<[**StatsUnsubscribeRequest**](StatsUnsubscribeRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_stats_subscribe_put

> models::CommonResponse basic_stats_subscribe_put(x_beam_gamertag, stats_subscribe_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stats_subscribe_request** | Option<[**StatsSubscribeRequest**](StatsSubscribeRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_stats_object_id_client_get

> models::StatsResponse object_stats_object_id_client_get(object_id, x_beam_gamertag, stats)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: domain.visibility.type.gamerTag | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stats** | Option<**String**> |  |  |

### Return type

[**models::StatsResponse**](StatsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_stats_object_id_client_post

> models::EmptyResponse object_stats_object_id_client_post(object_id, x_beam_gamertag, stat_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: domain.visibility.type.gamerTag | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stat_update_request** | Option<[**StatUpdateRequest**](StatUpdateRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_stats_object_id_client_stringlist_post

> models::EmptyResponse object_stats_object_id_client_stringlist_post(object_id, x_beam_gamertag, stat_update_request_string_list_format)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: domain.visibility.type.gamerTag | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stat_update_request_string_list_format** | Option<[**StatUpdateRequestStringListFormat**](StatUpdateRequestStringListFormat.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_stats_object_id_delete

> models::EmptyResponse object_stats_object_id_delete(object_id, x_beam_gamertag, stat_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: domain.visibility.type.gamerTag | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stat_request** | Option<[**StatRequest**](StatRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_stats_object_id_get

> models::StatsResponse object_stats_object_id_get(object_id, x_beam_gamertag, stats)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: domain.visibility.type.gamerTag | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stats** | Option<**String**> |  |  |

### Return type

[**models::StatsResponse**](StatsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_stats_object_id_post

> models::EmptyResponse object_stats_object_id_post(object_id, x_beam_gamertag, stat_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: domain.visibility.type.gamerTag | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stat_update_request** | Option<[**StatUpdateRequest**](StatUpdateRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

