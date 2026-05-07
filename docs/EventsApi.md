# \EventsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_events_apply_content_post**](EventsApi.md#basic_events_apply_content_post) | **POST** /basic/events/applyContent | 
[**basic_events_calendar_get**](EventsApi.md#basic_events_calendar_get) | **GET** /basic/events/calendar | 
[**basic_events_content_get**](EventsApi.md#basic_events_content_get) | **GET** /basic/events/content | 
[**basic_events_running_get**](EventsApi.md#basic_events_running_get) | **GET** /basic/events/running | 
[**object_events_object_id_content_delete**](EventsApi.md#object_events_object_id_content_delete) | **DELETE** /object/events/{objectId}/content | 
[**object_events_object_id_content_put**](EventsApi.md#object_events_object_id_content_put) | **PUT** /object/events/{objectId}/content | 
[**object_events_object_id_end_phase_put**](EventsApi.md#object_events_object_id_end_phase_put) | **PUT** /object/events/{objectId}/endPhase | 
[**object_events_object_id_get**](EventsApi.md#object_events_object_id_get) | **GET** /object/events/{objectId}/ | 
[**object_events_object_id_ping_get**](EventsApi.md#object_events_object_id_ping_get) | **GET** /object/events/{objectId}/ping | 
[**object_events_object_id_refresh_put**](EventsApi.md#object_events_object_id_refresh_put) | **PUT** /object/events/{objectId}/refresh | 



## basic_events_apply_content_post

> models::CommonResponse basic_events_apply_content_post(x_beam_gamertag, event_apply_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**event_apply_request** | Option<[**EventApplyRequest**](EventApplyRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_events_calendar_get

> models::EventsInDateRangeResponse basic_events_calendar_get(x_beam_gamertag, from, to, query, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |
**query** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::EventsInDateRangeResponse**](EventsInDateRangeResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_events_content_get

> models::EventContentResponse basic_events_content_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::EventContentResponse**](EventContentResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_events_running_get

> models::EventQueryResponse basic_events_running_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::EventQueryResponse**](EventQueryResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_events_object_id_content_delete

> models::CommonResponse object_events_object_id_content_delete(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: events.event_content_id.event_started_timestamp | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_events_object_id_content_put

> models::CommonResponse object_events_object_id_content_put(object_id, x_beam_gamertag, set_content_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: events.event_content_id.event_started_timestamp | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**set_content_request** | Option<[**SetContentRequest**](SetContentRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_events_object_id_end_phase_put

> models::CommonResponse object_events_object_id_end_phase_put(object_id, x_beam_gamertag, event_phase_end_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: events.event_content_id.event_started_timestamp | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**event_phase_end_request** | Option<[**EventPhaseEndRequest**](EventPhaseEndRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_events_object_id_get

> models::EventObjectData object_events_object_id_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: events.event_content_id.event_started_timestamp | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::EventObjectData**](EventObjectData.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_events_object_id_ping_get

> models::PingRsp object_events_object_id_ping_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: events.event_content_id.event_started_timestamp | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::PingRsp**](PingRsp.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_events_object_id_refresh_put

> models::CommonResponse object_events_object_id_refresh_put(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Format: events.event_content_id.event_started_timestamp | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

