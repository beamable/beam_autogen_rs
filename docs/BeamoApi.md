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

> models::BeamoActorManifest api_beamo_manifests_current_get(x_beam_scope, x_beam_gamertag, archived)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**archived** | Option<**bool**> |  |  |

### Return type

[**models::BeamoActorManifest**](BeamoActorManifest.md)

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

> models::BeamoActorGetManifestsResponse api_beamo_manifests_get(x_beam_scope, x_beam_gamertag, offset, limit, archived)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**offset** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**archived** | Option<**bool**> |  |  |

### Return type

[**models::BeamoActorGetManifestsResponse**](BeamoActorGetManifestsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_manifests_manifest_id_get

> models::BeamoActorManifest api_beamo_manifests_manifest_id_get(manifest_id, x_beam_scope, x_beam_gamertag, archived)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manifest_id** | **uuid::Uuid** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**archived** | Option<**bool**> |  |  |

### Return type

[**models::BeamoActorManifest**](BeamoActorManifest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_manifests_post

> models::BeamoActorManifestChecksum api_beamo_manifests_post(x_beam_scope, x_beam_gamertag, beamo_actor_post_manifest_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**beamo_actor_post_manifest_request** | Option<[**BeamoActorPostManifestRequest**](BeamoActorPostManifestRequest.md)> |  |  |

### Return type

[**models::BeamoActorManifestChecksum**](BeamoActorManifestChecksum.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_manifests_promote_post

> serde_json::Value api_beamo_manifests_promote_post(x_beam_scope, x_beam_gamertag, promote_beamo_manifest_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**promote_beamo_manifest_request** | Option<[**PromoteBeamoManifestRequest**](PromoteBeamoManifestRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_registry_uri_get

> models::UriResponse api_beamo_registry_uri_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::UriResponse**](UriResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_federation_post

> models::FederationRegistrationResponse api_beamo_services_federation_post(x_beam_scope, x_beam_gamertag, service_registration_query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**service_registration_query** | Option<[**ServiceRegistrationQuery**](ServiceRegistrationQuery.md)> |  |  |

### Return type

[**models::FederationRegistrationResponse**](FederationRegistrationResponse.md)

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

> models::SignedRequest api_beamo_services_logs_query_query_id_get(query_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::SignedRequest**](SignedRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_registrations_post

> models::ServiceRegistrationResponse api_beamo_services_registrations_post(x_beam_scope, x_beam_gamertag, service_registration_query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**service_registration_query** | Option<[**ServiceRegistrationQuery**](ServiceRegistrationQuery.md)> |  |  |

### Return type

[**models::ServiceRegistrationResponse**](ServiceRegistrationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_secret_get

> models::GetServiceSecretResponse api_beamo_services_secret_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::GetServiceSecretResponse**](GetServiceSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_service_name_federation_traffic_delete

> serde_json::Value api_beamo_services_service_name_federation_traffic_delete(service_name, x_beam_scope, x_beam_gamertag, delete_registration_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**delete_registration_request** | Option<[**DeleteRegistrationRequest**](DeleteRegistrationRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_service_name_federation_traffic_put

> serde_json::Value api_beamo_services_service_name_federation_traffic_put(service_name, x_beam_scope, x_beam_gamertag, service_registration_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**service_registration_request** | Option<[**ServiceRegistrationRequest**](ServiceRegistrationRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_service_name_logs_query_post

> models::QueryResponse api_beamo_services_service_name_logs_query_post(service_name, x_beam_scope, x_beam_gamertag, start_service_logs_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**start_service_logs_request** | Option<[**StartServiceLogsRequest**](StartServiceLogsRequest.md)> |  |  |

### Return type

[**models::QueryResponse**](QueryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_services_service_name_metrics_request_post

> models::SignedRequest api_beamo_services_service_name_metrics_request_post(service_name, x_beam_scope, x_beam_gamertag, get_metrics_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**get_metrics_request** | Option<[**GetMetricsRequest**](GetMetricsRequest.md)> |  |  |

### Return type

[**models::SignedRequest**](SignedRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_status_get

> models::BeamoActorGetStatusResponse api_beamo_status_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::BeamoActorGetStatusResponse**](BeamoActorGetStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_storage_connection_get

> models::ConnectionStringResponse api_beamo_storage_connection_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::ConnectionStringResponse**](ConnectionStringResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_storage_storage_object_name_performance_get

> models::StoragePerformance api_beamo_storage_storage_object_name_performance_get(storage_object_name, x_beam_scope, x_beam_gamertag, period, start_time, end_time, granularity)


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

[**models::StoragePerformance**](StoragePerformance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_beamo_templates_get

> models::BeamoActorGetTemplatesResponse api_beamo_templates_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::BeamoActorGetTemplatesResponse**](BeamoActorGetTemplatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

