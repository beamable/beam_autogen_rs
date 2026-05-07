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

> models::OtelAuthConfig api_beamo_otel_auth_reader_config_get(x_beam_gamertag, x_beam_timeout)


Get OTel authentication configuration for the ClickHouse reader role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::OtelAuthConfig**](OtelAuthConfig.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_auth_writer_config_get

> models::OtelAuthConfig api_beamo_otel_auth_writer_config_get(x_beam_gamertag, x_beam_timeout)


Get OTel authentication configuration for the ClickHouse writer role. Requires admin access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::OtelAuthConfig**](OtelAuthConfig.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_views_get

> models::OtelViewsResponse api_beamo_otel_views_get(x_beam_gamertag, x_beam_timeout, player_id)


Get available OpenTelemetry views for a specific player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**player_id** | Option<**String**> | Player ID to retrieve views for. |  |

### Return type

[**models::OtelViewsResponse**](OtelViewsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_views_post

> models::OtelView api_beamo_otel_views_post(x_beam_gamertag, x_beam_timeout, otel_view)


Save or update an OpenTelemetry view configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**otel_view** | Option<[**OtelView**](OtelView.md)> | The OTel view to save. |  |

### Return type

[**models::OtelView**](OtelView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_views_view_id_delete

> serde_json::Value api_beamo_otel_views_view_id_delete(view_id, x_beam_gamertag, x_beam_timeout)


Delete an OpenTelemetry view by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_id** | **String** | ID of the view to delete. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_otel_views_view_id_put

> models::OtelView api_beamo_otel_views_view_id_put(view_id, x_beam_gamertag, x_beam_timeout, update_otel_view_request)


Update an existing OpenTelemetry view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_id** | **String** | ID of the view to update. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**update_otel_view_request** | Option<[**UpdateOtelViewRequest**](UpdateOtelViewRequest.md)> | Fields to update on the view. |  |

### Return type

[**models::OtelView**](OtelView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

