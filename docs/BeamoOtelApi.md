# \BeamoOtelApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_beamo_otel_auth_reader_config_get**](BeamoOtelApi.md#api_beamo_otel_auth_reader_config_get) | **GET** /api/beamo/otel/auth/reader/config | 
[**api_beamo_otel_auth_writer_config_get**](BeamoOtelApi.md#api_beamo_otel_auth_writer_config_get) | **GET** /api/beamo/otel/auth/writer/config | 



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

