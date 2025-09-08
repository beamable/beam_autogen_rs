# \BeamoOtelApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_beamo_otel_auth_reader_config_get**](BeamoOtelApi.md#api_beamo_otel_auth_reader_config_get) | **GET** /api/beamo/otel/auth/reader/config | 
[**api_beamo_otel_auth_writer_config_get**](BeamoOtelApi.md#api_beamo_otel_auth_writer_config_get) | **GET** /api/beamo/otel/auth/writer/config | 
[**api_beamo_otel_views_get**](BeamoOtelApi.md#api_beamo_otel_views_get) | **GET** /api/beamo/otel/views | 
[**api_beamo_otel_views_post**](BeamoOtelApi.md#api_beamo_otel_views_post) | **POST** /api/beamo/otel/views | 
[**api_beamo_otel_views_view_id_delete**](BeamoOtelApi.md#api_beamo_otel_views_view_id_delete) | **DELETE** /api/beamo/otel/views/{viewId} | 
[**api_beamo_otel_views_view_id_put**](BeamoOtelApi.md#api_beamo_otel_views_view_id_put) | **PUT** /api/beamo/otel/views/{viewId} | 



## api_beamo_otel_auth_reader_config_get

> models::OtelAuthConfig api_beamo_otel_auth_reader_config_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::OtelAuthConfig**](OtelAuthConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_auth_writer_config_get

> models::OtelAuthConfig api_beamo_otel_auth_writer_config_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::OtelAuthConfig**](OtelAuthConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_views_get

> models::OtelViewsResponse api_beamo_otel_views_get(x_beam_scope, x_beam_gamertag, player_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**player_id** | Option<**String**> |  |  |

### Return type

[**models::OtelViewsResponse**](OtelViewsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_views_post

> models::OtelView api_beamo_otel_views_post(x_beam_scope, x_beam_gamertag, otel_view)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**otel_view** | Option<[**OtelView**](OtelView.md)> |  |  |

### Return type

[**models::OtelView**](OtelView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_views_view_id_delete

> serde_json::Value api_beamo_otel_views_view_id_delete(view_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_views_view_id_put

> models::OtelView api_beamo_otel_views_view_id_put(view_id, x_beam_scope, x_beam_gamertag, update_otel_view_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**update_otel_view_request** | Option<[**UpdateOtelViewRequest**](UpdateOtelViewRequest.md)> |  |  |

### Return type

[**models::OtelView**](OtelView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

