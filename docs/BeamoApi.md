# \BeamoApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_beamo_manifests_current_get**](BeamoApi.md#api_beamo_manifests_current_get) | **GET** /api/beamo/manifests/current | 
[**api_beamo_manifests_current_post**](BeamoApi.md#api_beamo_manifests_current_post) | **POST** /api/beamo/manifests/current | 
[**api_beamo_manifests_get**](BeamoApi.md#api_beamo_manifests_get) | **GET** /api/beamo/manifests | 
[**api_beamo_manifests_manifest_id_get**](BeamoApi.md#api_beamo_manifests_manifest_id_get) | **GET** /api/beamo/manifests/{manifestId} | 
[**api_beamo_manifests_post**](BeamoApi.md#api_beamo_manifests_post) | **POST** /api/beamo/manifests | 
[**api_beamo_manifests_promote_post**](BeamoApi.md#api_beamo_manifests_promote_post) | **POST** /api/beamo/manifests/promote | 
[**api_beamo_registry_uri_get**](BeamoApi.md#api_beamo_registry_uri_get) | **GET** /api/beamo/registry-uri | 
[**api_beamo_services_federation_post**](BeamoApi.md#api_beamo_services_federation_post) | **POST** /api/beamo/services/federation | 
[**api_beamo_services_logs_query_query_id_delete**](BeamoApi.md#api_beamo_services_logs_query_query_id_delete) | **DELETE** /api/beamo/services/logs/query/{queryId} | 
[**api_beamo_services_logs_query_query_id_get**](BeamoApi.md#api_beamo_services_logs_query_query_id_get) | **GET** /api/beamo/services/logs/query/{queryId} | 
[**api_beamo_services_registrations_post**](BeamoApi.md#api_beamo_services_registrations_post) | **POST** /api/beamo/services/registrations | 
[**api_beamo_services_secret_get**](BeamoApi.md#api_beamo_services_secret_get) | **GET** /api/beamo/services/secret | 
[**api_beamo_services_service_name_federation_traffic_delete**](BeamoApi.md#api_beamo_services_service_name_federation_traffic_delete) | **DELETE** /api/beamo/services/{serviceName}/federation/traffic | 
[**api_beamo_services_service_name_federation_traffic_put**](BeamoApi.md#api_beamo_services_service_name_federation_traffic_put) | **PUT** /api/beamo/services/{serviceName}/federation/traffic | 
[**api_beamo_services_service_name_logs_query_post**](BeamoApi.md#api_beamo_services_service_name_logs_query_post) | **POST** /api/beamo/services/{serviceName}/logs/query | 
[**api_beamo_services_service_name_metrics_request_post**](BeamoApi.md#api_beamo_services_service_name_metrics_request_post) | **POST** /api/beamo/services/{serviceName}/metrics-request | 
[**api_beamo_status_get**](BeamoApi.md#api_beamo_status_get) | **GET** /api/beamo/status | 
[**api_beamo_storage_connection_get**](BeamoApi.md#api_beamo_storage_connection_get) | **GET** /api/beamo/storage/connection | 
[**api_beamo_storage_storage_object_name_performance_get**](BeamoApi.md#api_beamo_storage_storage_object_name_performance_get) | **GET** /api/beamo/storage/{storageObjectName}/performance | 
[**api_beamo_templates_get**](BeamoApi.md#api_beamo_templates_get) | **GET** /api/beamo/templates | 



## api_beamo_manifests_current_get

> models::BeamoV2Manifest api_beamo_manifests_current_get(x_beam_scope, x_beam_gamertag, archived)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**archived** | Option<**bool**> |  |  |

### Return type

[**models::BeamoV2Manifest**](BeamoV2Manifest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_manifests_current_post

> serde_json::Value api_beamo_manifests_current_post(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## api_beamo_manifests_get

> models::BeamoV2GetManifestsResponse api_beamo_manifests_get(x_beam_scope, x_beam_gamertag, offset, limit, archived)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**offset** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**archived** | Option<**bool**> |  |  |

### Return type

[**models::BeamoV2GetManifestsResponse**](BeamoV2GetManifestsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_manifests_manifest_id_get

> models::BeamoV2Manifest api_beamo_manifests_manifest_id_get(manifest_id, x_beam_scope, x_beam_gamertag, archived)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manifest_id** | **uuid::Uuid** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**archived** | Option<**bool**> |  |  |

### Return type

[**models::BeamoV2Manifest**](BeamoV2Manifest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_manifests_post

> models::BeamoV2ManifestChecksum api_beamo_manifests_post(x_beam_scope, x_beam_gamertag, beamo_v2_post_manifest_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_v2_post_manifest_request** | Option<[**BeamoV2PostManifestRequest**](BeamoV2PostManifestRequest.md)> |  |  |

### Return type

[**models::BeamoV2ManifestChecksum**](BeamoV2ManifestChecksum.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_manifests_promote_post

> serde_json::Value api_beamo_manifests_promote_post(x_beam_scope, x_beam_gamertag, beamo_v2_promote_beamo_manifest_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_v2_promote_beamo_manifest_request** | Option<[**BeamoV2PromoteBeamoManifestRequest**](BeamoV2PromoteBeamoManifestRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_registry_uri_get

> models::BeamoV2UriResponse api_beamo_registry_uri_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::BeamoV2UriResponse**](BeamoV2UriResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_federation_post

> models::BeamoV2FederationRegistrationResponse api_beamo_services_federation_post(x_beam_scope, x_beam_gamertag, beamo_v2_service_registration_query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_v2_service_registration_query** | Option<[**BeamoV2ServiceRegistrationQuery**](BeamoV2ServiceRegistrationQuery.md)> |  |  |

### Return type

[**models::BeamoV2FederationRegistrationResponse**](BeamoV2FederationRegistrationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_logs_query_query_id_delete

> serde_json::Value api_beamo_services_logs_query_query_id_delete(query_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **String** |  | [required] |
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


## api_beamo_services_logs_query_query_id_get

> models::BeamoV2SignedRequest api_beamo_services_logs_query_query_id_get(query_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::BeamoV2SignedRequest**](BeamoV2SignedRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_registrations_post

> models::BeamoV2ServiceRegistrationResponse api_beamo_services_registrations_post(x_beam_scope, x_beam_gamertag, beamo_v2_service_registration_query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_v2_service_registration_query** | Option<[**BeamoV2ServiceRegistrationQuery**](BeamoV2ServiceRegistrationQuery.md)> |  |  |

### Return type

[**models::BeamoV2ServiceRegistrationResponse**](BeamoV2ServiceRegistrationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_secret_get

> models::BeamoV2GetServiceSecretResponse api_beamo_services_secret_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::BeamoV2GetServiceSecretResponse**](BeamoV2GetServiceSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_service_name_federation_traffic_delete

> serde_json::Value api_beamo_services_service_name_federation_traffic_delete(service_name, x_beam_scope, x_beam_gamertag, beamo_v2_delete_registration_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_v2_delete_registration_request** | Option<[**BeamoV2DeleteRegistrationRequest**](BeamoV2DeleteRegistrationRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_service_name_federation_traffic_put

> serde_json::Value api_beamo_services_service_name_federation_traffic_put(service_name, x_beam_scope, x_beam_gamertag, beamo_v2_service_registration_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_v2_service_registration_request** | Option<[**BeamoV2ServiceRegistrationRequest**](BeamoV2ServiceRegistrationRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_service_name_logs_query_post

> models::BeamoV2QueryResponse api_beamo_services_service_name_logs_query_post(service_name, x_beam_scope, x_beam_gamertag, beamo_v2_start_service_logs_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_v2_start_service_logs_request** | Option<[**BeamoV2StartServiceLogsRequest**](BeamoV2StartServiceLogsRequest.md)> |  |  |

### Return type

[**models::BeamoV2QueryResponse**](BeamoV2QueryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_service_name_metrics_request_post

> models::BeamoV2SignedRequest api_beamo_services_service_name_metrics_request_post(service_name, x_beam_scope, x_beam_gamertag, beamo_v2_get_metrics_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_v2_get_metrics_request** | Option<[**BeamoV2GetMetricsRequest**](BeamoV2GetMetricsRequest.md)> |  |  |

### Return type

[**models::BeamoV2SignedRequest**](BeamoV2SignedRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_status_get

> models::BeamoV2GetStatusResponse api_beamo_status_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::BeamoV2GetStatusResponse**](BeamoV2GetStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_storage_connection_get

> models::BeamoV2ConnectionStringResponse api_beamo_storage_connection_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::BeamoV2ConnectionStringResponse**](BeamoV2ConnectionStringResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_storage_storage_object_name_performance_get

> models::BeamoV2StoragePerformance api_beamo_storage_storage_object_name_performance_get(storage_object_name, x_beam_scope, x_beam_gamertag, period, start_time, end_time, granularity)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_object_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**period** | Option<**String**> |  |  |
**start_time** | Option<**String**> |  |  |
**end_time** | Option<**String**> |  |  |
**granularity** | Option<**String**> |  |  |

### Return type

[**models::BeamoV2StoragePerformance**](BeamoV2StoragePerformance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_templates_get

> models::BeamoV2GetTemplatesResponse api_beamo_templates_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::BeamoV2GetTemplatesResponse**](BeamoV2GetTemplatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

