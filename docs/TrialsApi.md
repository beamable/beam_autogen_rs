# \TrialsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_trials_admin_data_get**](TrialsApi.md#basic_trials_admin_data_get) | **GET** /basic/trials/admin/data | 
[**basic_trials_admin_get**](TrialsApi.md#basic_trials_admin_get) | **GET** /basic/trials/admin | 
[**basic_trials_data_delete**](TrialsApi.md#basic_trials_data_delete) | **DELETE** /basic/trials/data | 
[**basic_trials_data_post**](TrialsApi.md#basic_trials_data_post) | **POST** /basic/trials/data | 
[**basic_trials_delete**](TrialsApi.md#basic_trials_delete) | **DELETE** /basic/trials/ | 
[**basic_trials_get**](TrialsApi.md#basic_trials_get) | **GET** /basic/trials/ | 
[**basic_trials_pause_put**](TrialsApi.md#basic_trials_pause_put) | **PUT** /basic/trials/pause | 
[**basic_trials_post**](TrialsApi.md#basic_trials_post) | **POST** /basic/trials/ | 
[**basic_trials_schedule_put**](TrialsApi.md#basic_trials_schedule_put) | **PUT** /basic/trials/schedule | 
[**basic_trials_start_put**](TrialsApi.md#basic_trials_start_put) | **PUT** /basic/trials/start | 



## basic_trials_admin_data_get

> models::GetS3DataResponse basic_trials_admin_data_get(id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetS3DataResponse**](GetS3DataResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_admin_get

> models::GetPlayerTrialsResponse basic_trials_admin_get(dbid, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbid** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetPlayerTrialsResponse**](GetPlayerTrialsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_data_delete

> models::TrialSuccessResponse basic_trials_data_delete(x_beam_gamertag, delete_trial_data_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_trial_data_request** | Option<[**DeleteTrialDataRequest**](DeleteTrialDataRequest.md)> |  |  |

### Return type

[**models::TrialSuccessResponse**](TrialSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_data_post

> models::SaveGameDataResponse basic_trials_data_post(x_beam_gamertag, upload_trial_data_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**upload_trial_data_request** | Option<[**UploadTrialDataRequest**](UploadTrialDataRequest.md)> |  |  |

### Return type

[**models::SaveGameDataResponse**](SaveGameDataResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_delete

> models::TrialSuccessResponse basic_trials_delete(x_beam_gamertag, delete_trial_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_trial_request** | Option<[**DeleteTrialRequest**](DeleteTrialRequest.md)> |  |  |

### Return type

[**models::TrialSuccessResponse**](TrialSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_get

> models::ListTrialsResponse basic_trials_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ListTrialsResponse**](ListTrialsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_pause_put

> models::TrialSuccessResponse basic_trials_pause_put(x_beam_gamertag, pause_trial_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**pause_trial_request** | Option<[**PauseTrialRequest**](PauseTrialRequest.md)> |  |  |

### Return type

[**models::TrialSuccessResponse**](TrialSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_post

> models::TrialSuccessResponse basic_trials_post(x_beam_gamertag, create_trial_rest_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_trial_rest_request** | Option<[**CreateTrialRestRequest**](CreateTrialRestRequest.md)> |  |  |

### Return type

[**models::TrialSuccessResponse**](TrialSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_schedule_put

> models::TrialSuccessResponse basic_trials_schedule_put(x_beam_gamertag, schedule_trial_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**schedule_trial_request** | Option<[**ScheduleTrialRequest**](ScheduleTrialRequest.md)> |  |  |

### Return type

[**models::TrialSuccessResponse**](TrialSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_trials_start_put

> models::TrialSuccessResponse basic_trials_start_put(x_beam_gamertag, start_trial_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**start_trial_request** | Option<[**StartTrialRequest**](StartTrialRequest.md)> |  |  |

### Return type

[**models::TrialSuccessResponse**](TrialSuccessResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

