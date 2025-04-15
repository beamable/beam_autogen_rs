# \SchedulerApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_internal_scheduler_job_execute_post**](SchedulerApi.md#api_internal_scheduler_job_execute_post) | **POST** /api/internal/scheduler/job/execute | 
[**api_internal_scheduler_job_post**](SchedulerApi.md#api_internal_scheduler_job_post) | **POST** /api/internal/scheduler/job | 
[**api_scheduler_job_job_id_activity_get**](SchedulerApi.md#api_scheduler_job_job_id_activity_get) | **GET** /api/scheduler/job/{jobId}/activity | 
[**api_scheduler_job_job_id_activity_paged_get**](SchedulerApi.md#api_scheduler_job_job_id_activity_paged_get) | **GET** /api/scheduler/job/{jobId}/activity-paged | 
[**api_scheduler_job_job_id_cancel_put**](SchedulerApi.md#api_scheduler_job_job_id_cancel_put) | **PUT** /api/scheduler/job/{jobId}/cancel | 
[**api_scheduler_job_job_id_delete**](SchedulerApi.md#api_scheduler_job_job_id_delete) | **DELETE** /api/scheduler/job/{jobId} | 
[**api_scheduler_job_job_id_get**](SchedulerApi.md#api_scheduler_job_job_id_get) | **GET** /api/scheduler/job/{jobId} | 
[**api_scheduler_job_job_id_next_executions_get**](SchedulerApi.md#api_scheduler_job_job_id_next_executions_get) | **GET** /api/scheduler/job/{jobId}/next-executions | 
[**api_scheduler_job_post**](SchedulerApi.md#api_scheduler_job_post) | **POST** /api/scheduler/job | 
[**api_scheduler_jobs_activity_paged_get**](SchedulerApi.md#api_scheduler_jobs_activity_paged_get) | **GET** /api/scheduler/jobs/activity-paged | 
[**api_scheduler_jobs_get**](SchedulerApi.md#api_scheduler_jobs_get) | **GET** /api/scheduler/jobs | 
[**api_scheduler_jobs_paged_get**](SchedulerApi.md#api_scheduler_jobs_paged_get) | **GET** /api/scheduler/jobs-paged | 
[**api_scheduler_jobs_suspended_get**](SchedulerApi.md#api_scheduler_jobs_suspended_get) | **GET** /api/scheduler/jobs/suspended | 



## api_internal_scheduler_job_execute_post

> models::JobExecutionResult api_internal_scheduler_job_execute_post(x_beam_scope, x_beam_gamertag, job_execution_event)


Called by the Dispatcher lambda function to start a job execution at the appropriate time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**job_execution_event** | Option<[**JobExecutionEvent**](JobExecutionEvent.md)> |  |  |

### Return type

[**models::JobExecutionResult**](JobExecutionResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_internal_scheduler_job_post

> models::JobDefinitionView api_internal_scheduler_job_post(x_beam_scope, x_beam_gamertag, job_definition_save_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**job_definition_save_request** | Option<[**JobDefinitionSaveRequest**](JobDefinitionSaveRequest.md)> |  |  |

### Return type

[**models::JobDefinitionView**](JobDefinitionView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_activity_get

> Vec<models::JobActivity> api_scheduler_job_job_id_activity_get(job_id, x_beam_scope, x_beam_gamertag, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**limit** | Option<**i32**> |  |  |[default to 1000]

### Return type

[**Vec<models::JobActivity>**](JobActivity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_activity_paged_get

> models::JobActivityViewCursorPagedResult api_scheduler_job_job_id_activity_paged_get(job_id, x_beam_scope, x_beam_gamertag, cursor)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**cursor** | Option<**String**> |  |  |

### Return type

[**models::JobActivityViewCursorPagedResult**](JobActivityViewCursorPagedResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_cancel_put

> serde_json::Value api_scheduler_job_job_id_cancel_put(job_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
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


## api_scheduler_job_job_id_delete

> serde_json::Value api_scheduler_job_job_id_delete(job_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
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


## api_scheduler_job_job_id_get

> models::JobDefinitionView api_scheduler_job_job_id_get(job_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::JobDefinitionView**](JobDefinitionView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_next_executions_get

> Vec<String> api_scheduler_job_job_id_next_executions_get(job_id, x_beam_scope, x_beam_gamertag, from, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**from** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 200]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_post

> models::JobDefinitionView api_scheduler_job_post(x_beam_scope, x_beam_gamertag, job_definition_save_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**job_definition_save_request** | Option<[**JobDefinitionSaveRequest**](JobDefinitionSaveRequest.md)> |  |  |

### Return type

[**models::JobDefinitionView**](JobDefinitionView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_activity_paged_get

> models::JobActivityViewCursorPagedResult api_scheduler_jobs_activity_paged_get(x_beam_scope, x_beam_gamertag, cursor)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**cursor** | Option<**String**> |  |  |

### Return type

[**models::JobActivityViewCursorPagedResult**](JobActivityViewCursorPagedResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_get

> Vec<models::JobDefinition> api_scheduler_jobs_get(x_beam_scope, x_beam_gamertag, source, name, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**source** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 1000]

### Return type

[**Vec<models::JobDefinition>**](JobDefinition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_paged_get

> models::JobDefinitionViewCursorPagedResult api_scheduler_jobs_paged_get(x_beam_scope, x_beam_gamertag, source, name, only_unique, cursor)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**source** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**only_unique** | Option<**bool**> |  |  |[default to false]
**cursor** | Option<**String**> |  |  |

### Return type

[**models::JobDefinitionViewCursorPagedResult**](JobDefinitionViewCursorPagedResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_suspended_get

> models::JobDefinitionViewCursorPagedResult api_scheduler_jobs_suspended_get(x_beam_scope, x_beam_gamertag, from, cursor)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**from** | Option<**String**> |  |  |
**cursor** | Option<**String**> |  |  |

### Return type

[**models::JobDefinitionViewCursorPagedResult**](JobDefinitionViewCursorPagedResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

