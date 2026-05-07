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

> models::JobExecutionResult api_internal_scheduler_job_execute_post(x_beam_gamertag, x_beam_timeout, execute_job_request)


Called by the Dispatcher lambda function to start a job execution at the appropriate time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**execute_job_request** | Option<[**ExecuteJobRequest**](ExecuteJobRequest.md)> | Job execution request from the Dispatcher Lambda. |  |

### Return type

[**models::JobExecutionResult**](JobExecutionResult.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_internal_scheduler_job_post

> models::JobDefinitionView api_internal_scheduler_job_post(x_beam_gamertag, x_beam_timeout, job_definition_save_request)


Create or update a job definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**job_definition_save_request** | Option<[**JobDefinitionSaveRequest**](JobDefinitionSaveRequest.md)> | Job definition to create or update. |  |

### Return type

[**models::JobDefinitionView**](JobDefinitionView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_activity_get

> Vec<models::JobActivity> api_scheduler_job_job_id_activity_get(job_id, x_beam_gamertag, x_beam_timeout, limit)


List activity records for a specific job. DEPRECATED: Use job/{jobId}/activity-paged instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of the job to retrieve activity for. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**limit** | Option<**i32**> | Maximum number of results. Cannot exceed 10000. |  |[default to 1000]

### Return type

[**Vec<models::JobActivity>**](JobActivity.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_activity_paged_get

> models::JobActivityViewCursorPagedResult api_scheduler_job_job_id_activity_paged_get(job_id, x_beam_gamertag, x_beam_timeout, cursor)


List activity records for a specific job with cursor-based pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of the job to retrieve activity for. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**cursor** | Option<**String**> | Pagination cursor from a previous response. |  |

### Return type

[**models::JobActivityViewCursorPagedResult**](JobActivityViewCursorPagedResult.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_cancel_put

> serde_json::Value api_scheduler_job_job_id_cancel_put(job_id, x_beam_gamertag, x_beam_timeout)


Cancel a job's triggers, preventing future executions without deleting the job definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of the job to cancel. | [required] |
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


## api_scheduler_job_job_id_delete

> serde_json::Value api_scheduler_job_job_id_delete(job_id, x_beam_gamertag, x_beam_timeout)


Delete a job definition and remove it from the scheduler.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of the job to delete. | [required] |
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


## api_scheduler_job_job_id_get

> models::JobDefinitionView api_scheduler_job_job_id_get(job_id, x_beam_gamertag, x_beam_timeout)


Get a single job definition by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of the job to retrieve. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::JobDefinitionView**](JobDefinitionView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_next_executions_get

> Vec<chrono::DateTime<chrono::FixedOffset>> api_scheduler_job_job_id_next_executions_get(job_id, x_beam_gamertag, x_beam_timeout, from, limit)


Preview the next scheduled execution times for a job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of the job to compute schedules for. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**from** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Start time for the schedule preview. Defaults to now. |  |
**limit** | Option<**i32**> | Maximum number of executions to return. Cannot exceed 1000. |  |[default to 200]

### Return type

[**Vec<chrono::DateTime<chrono::FixedOffset>>**](chrono::DateTime<chrono::FixedOffset>.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_post

> models::JobDefinitionView api_scheduler_job_post(x_beam_gamertag, x_beam_timeout, job_definition_save_request)


Create or update a job definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**job_definition_save_request** | Option<[**JobDefinitionSaveRequest**](JobDefinitionSaveRequest.md)> | Job definition to create or update. |  |

### Return type

[**models::JobDefinitionView**](JobDefinitionView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_activity_paged_get

> models::JobActivityViewCursorPagedResult api_scheduler_jobs_activity_paged_get(x_beam_gamertag, x_beam_timeout, cursor)


List activity records across all jobs in the realm with cursor-based pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**cursor** | Option<**String**> | Pagination cursor from a previous response. |  |

### Return type

[**models::JobActivityViewCursorPagedResult**](JobActivityViewCursorPagedResult.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_get

> Vec<models::JobDefinition> api_scheduler_jobs_get(x_beam_gamertag, x_beam_timeout, source, name, limit)


List job definitions for the current realm. DEPRECATED: Use jobs-paged instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**source** | Option<**String**> | Optional source filter. |  |
**name** | Option<**String**> | Optional name filter. |  |
**limit** | Option<**i32**> | Maximum number of results. Cannot exceed 10000. |  |[default to 1000]

### Return type

[**Vec<models::JobDefinition>**](JobDefinition.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_paged_get

> models::JobDefinitionViewCursorPagedResult api_scheduler_jobs_paged_get(x_beam_gamertag, x_beam_timeout, source, name, only_unique, cursor)


List job definitions for the current realm with cursor-based pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**source** | Option<**String**> | Optional source filter. |  |
**name** | Option<**String**> | Optional name filter. |  |
**only_unique** | Option<**bool**> | When true, only returns unique jobs. |  |[default to false]
**cursor** | Option<**String**> | Pagination cursor from a previous response. |  |

### Return type

[**models::JobDefinitionViewCursorPagedResult**](JobDefinitionViewCursorPagedResult.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_suspended_get

> models::JobDefinitionViewCursorPagedResult api_scheduler_jobs_suspended_get(x_beam_gamertag, x_beam_timeout, from, cursor)


List suspended job definitions with cursor-based pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**from** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Return jobs suspended from this datetime. Mutually exclusive with cursor. |  |
**cursor** | Option<**String**> | Pagination cursor from a previous response. Mutually exclusive with from. |  |

### Return type

[**models::JobDefinitionViewCursorPagedResult**](JobDefinitionViewCursorPagedResult.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

