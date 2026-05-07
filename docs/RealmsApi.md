# \RealmsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_realms_admin_customer_get**](RealmsApi.md#basic_realms_admin_customer_get) | **GET** /basic/realms/admin/customer | 
[**basic_realms_admin_inflight_failures_delete**](RealmsApi.md#basic_realms_admin_inflight_failures_delete) | **DELETE** /basic/realms/admin/inflight/failures | 
[**basic_realms_admin_inflight_failures_get**](RealmsApi.md#basic_realms_admin_inflight_failures_get) | **GET** /basic/realms/admin/inflight/failures | 
[**basic_realms_client_defaults_get**](RealmsApi.md#basic_realms_client_defaults_get) | **GET** /basic/realms/client/defaults | 
[**basic_realms_config_get**](RealmsApi.md#basic_realms_config_get) | **GET** /basic/realms/config | 
[**basic_realms_config_post**](RealmsApi.md#basic_realms_config_post) | **POST** /basic/realms/config | 
[**basic_realms_config_put**](RealmsApi.md#basic_realms_config_put) | **PUT** /basic/realms/config | 
[**basic_realms_customer_activate_get**](RealmsApi.md#basic_realms_customer_activate_get) | **GET** /basic/realms/customer/activate | 
[**basic_realms_customer_alias_available_get**](RealmsApi.md#basic_realms_customer_alias_available_get) | **GET** /basic/realms/customer/alias/available | 
[**basic_realms_customer_get**](RealmsApi.md#basic_realms_customer_get) | **GET** /basic/realms/customer | 
[**basic_realms_customer_post**](RealmsApi.md#basic_realms_customer_post) | **POST** /basic/realms/customer | 
[**basic_realms_customer_verify_post**](RealmsApi.md#basic_realms_customer_verify_post) | **POST** /basic/realms/customer/verify | 
[**basic_realms_customers_get**](RealmsApi.md#basic_realms_customers_get) | **GET** /basic/realms/customers | 
[**basic_realms_game_get**](RealmsApi.md#basic_realms_game_get) | **GET** /basic/realms/game | 
[**basic_realms_game_post**](RealmsApi.md#basic_realms_game_post) | **POST** /basic/realms/game | 
[**basic_realms_game_put**](RealmsApi.md#basic_realms_game_put) | **PUT** /basic/realms/game | 
[**basic_realms_games_get**](RealmsApi.md#basic_realms_games_get) | **GET** /basic/realms/games | 
[**basic_realms_is_customer_get**](RealmsApi.md#basic_realms_is_customer_get) | **GET** /basic/realms/is-customer | 
[**basic_realms_launch_message_delete**](RealmsApi.md#basic_realms_launch_message_delete) | **DELETE** /basic/realms/launch-message | 
[**basic_realms_launch_message_get**](RealmsApi.md#basic_realms_launch_message_get) | **GET** /basic/realms/launch-message | 
[**basic_realms_launch_message_post**](RealmsApi.md#basic_realms_launch_message_post) | **POST** /basic/realms/launch-message | 
[**basic_realms_plans_get**](RealmsApi.md#basic_realms_plans_get) | **GET** /basic/realms/plans | 
[**basic_realms_plans_post**](RealmsApi.md#basic_realms_plans_post) | **POST** /basic/realms/plans | 
[**basic_realms_project_beamable_post**](RealmsApi.md#basic_realms_project_beamable_post) | **POST** /basic/realms/project/beamable | 
[**basic_realms_project_delete**](RealmsApi.md#basic_realms_project_delete) | **DELETE** /basic/realms/project | 
[**basic_realms_project_get**](RealmsApi.md#basic_realms_project_get) | **GET** /basic/realms/project | 
[**basic_realms_project_post**](RealmsApi.md#basic_realms_project_post) | **POST** /basic/realms/project | 
[**basic_realms_project_promote_get**](RealmsApi.md#basic_realms_project_promote_get) | **GET** /basic/realms/project/promote | 
[**basic_realms_project_promote_post**](RealmsApi.md#basic_realms_project_promote_post) | **POST** /basic/realms/project/promote | 
[**basic_realms_project_put**](RealmsApi.md#basic_realms_project_put) | **PUT** /basic/realms/project | 
[**basic_realms_project_rename_put**](RealmsApi.md#basic_realms_project_rename_put) | **PUT** /basic/realms/project/rename | 
[**basic_realms_promotion_get**](RealmsApi.md#basic_realms_promotion_get) | **GET** /basic/realms/promotion | 
[**basic_realms_promotion_post**](RealmsApi.md#basic_realms_promotion_post) | **POST** /basic/realms/promotion | 



## basic_realms_admin_customer_get

> models::CustomerResponse basic_realms_admin_customer_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CustomerResponse**](CustomerResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_admin_inflight_failures_delete

> models::CommonResponse basic_realms_admin_inflight_failures_delete(x_beam_gamertag, batch_delete_in_flight_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**batch_delete_in_flight_request** | Option<[**BatchDeleteInFlightRequest**](BatchDeleteInFlightRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_admin_inflight_failures_get

> models::InFlightFailureResponse basic_realms_admin_inflight_failures_get(service_name, x_beam_gamertag, service_object_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**service_object_id** | Option<**String**> |  |  |

### Return type

[**models::InFlightFailureResponse**](InFlightFailureResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_client_defaults_get

> models::RealmsBasicRealmConfiguration basic_realms_client_defaults_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::RealmsBasicRealmConfiguration**](RealmsBasicRealmConfiguration.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_config_get

> models::RealmsBasicRealmConfigResponse basic_realms_config_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::RealmsBasicRealmConfigResponse**](RealmsBasicRealmConfigResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_config_post

> models::CommonResponse basic_realms_config_post(x_beam_gamertag, realm_config_change_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**realm_config_change_request** | Option<[**RealmConfigChangeRequest**](RealmConfigChangeRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_config_put

> models::CommonResponse basic_realms_config_put(x_beam_gamertag, realms_basic_realm_config_save_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**realms_basic_realm_config_save_request** | Option<[**RealmsBasicRealmConfigSaveRequest**](RealmsBasicRealmConfigSaveRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_customer_activate_get

> models::HtmlResponse basic_realms_customer_activate_get(token, cid, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**cid** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::HtmlResponse**](HtmlResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_customer_alias_available_get

> models::RealmsBasicAliasAvailableResponse basic_realms_customer_alias_available_get(alias, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::RealmsBasicAliasAvailableResponse**](RealmsBasicAliasAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_customer_get

> models::CustomerViewResponse basic_realms_customer_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CustomerViewResponse**](CustomerViewResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_customer_post

> models::RealmsBasicNewCustomerResponse basic_realms_customer_post(x_beam_gamertag, realms_basic_new_customer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**realms_basic_new_customer_request** | Option<[**RealmsBasicNewCustomerRequest**](RealmsBasicNewCustomerRequest.md)> |  |  |

### Return type

[**models::RealmsBasicNewCustomerResponse**](RealmsBasicNewCustomerResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_customer_verify_post

> models::RealmsBasicNewCustomerResponse basic_realms_customer_verify_post(x_beam_gamertag, realms_basic_new_customer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**realms_basic_new_customer_request** | Option<[**RealmsBasicNewCustomerRequest**](RealmsBasicNewCustomerRequest.md)> |  |  |

### Return type

[**models::RealmsBasicNewCustomerResponse**](RealmsBasicNewCustomerResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_customers_get

> models::RealmsBasicCustomersResponse basic_realms_customers_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::RealmsBasicCustomersResponse**](RealmsBasicCustomersResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_game_get

> models::GetGameResponse basic_realms_game_get(root_pid, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_pid** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetGameResponse**](GetGameResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_game_post

> models::CommonResponse basic_realms_game_post(x_beam_gamertag, realms_basic_new_game_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**realms_basic_new_game_request** | Option<[**RealmsBasicNewGameRequest**](RealmsBasicNewGameRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_game_put

> models::CommonResponse basic_realms_game_put(x_beam_gamertag, realms_basic_update_game_hierarchy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**realms_basic_update_game_hierarchy_request** | Option<[**RealmsBasicUpdateGameHierarchyRequest**](RealmsBasicUpdateGameHierarchyRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_games_get

> models::GetGameResponse basic_realms_games_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetGameResponse**](GetGameResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_is_customer_get

> models::EmptyResponse basic_realms_is_customer_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_launch_message_delete

> models::CommonResponse basic_realms_launch_message_delete(x_beam_gamertag, remove_launch_message_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**remove_launch_message_request** | Option<[**RemoveLaunchMessageRequest**](RemoveLaunchMessageRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_launch_message_get

> models::LaunchMessageListResponse basic_realms_launch_message_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::LaunchMessageListResponse**](LaunchMessageListResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_launch_message_post

> models::CommonResponse basic_realms_launch_message_post(x_beam_gamertag, create_launch_message_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_launch_message_request** | Option<[**CreateLaunchMessageRequest**](CreateLaunchMessageRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_plans_get

> models::ServicePlansResponse basic_realms_plans_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ServicePlansResponse**](ServicePlansResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_plans_post

> models::CommonResponse basic_realms_plans_post(x_beam_gamertag, create_plan_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_plan_request** | Option<[**CreatePlanRequest**](CreatePlanRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_project_beamable_post

> models::CommonResponse basic_realms_project_beamable_post(x_beam_gamertag, create_project_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_project_request** | Option<[**CreateProjectRequest**](CreateProjectRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_project_delete

> models::CommonResponse basic_realms_project_delete(x_beam_gamertag, archive_project_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**archive_project_request** | Option<[**ArchiveProjectRequest**](ArchiveProjectRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_project_get

> models::ProjectView basic_realms_project_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ProjectView**](ProjectView.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_project_post

> models::CommonResponse basic_realms_project_post(x_beam_gamertag, create_project_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_project_request** | Option<[**CreateProjectRequest**](CreateProjectRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_project_promote_get

> models::PromoteRealmResponseOld basic_realms_project_promote_get(source_pid, x_beam_gamertag, promotions, content_manifest_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_pid** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**promotions** | Option<[**Vec<String>**](String.md)> |  |  |
**content_manifest_ids** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::PromoteRealmResponseOld**](PromoteRealmResponseOld.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_project_promote_post

> models::PromoteRealmResponseOld basic_realms_project_promote_post(x_beam_gamertag, realms_basic_promote_realm_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**realms_basic_promote_realm_request** | Option<[**RealmsBasicPromoteRealmRequest**](RealmsBasicPromoteRealmRequest.md)> |  |  |

### Return type

[**models::PromoteRealmResponseOld**](PromoteRealmResponseOld.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_project_put

> models::CommonResponse basic_realms_project_put(x_beam_gamertag, unarchive_project_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**unarchive_project_request** | Option<[**UnarchiveProjectRequest**](UnarchiveProjectRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_project_rename_put

> models::CommonResponse basic_realms_project_rename_put(x_beam_gamertag, rename_project_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**rename_project_request** | Option<[**RenameProjectRequest**](RenameProjectRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_promotion_get

> models::RealmsBasicPromoteRealmResponse basic_realms_promotion_get(source_pid, x_beam_gamertag, promotions, content_manifest_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_pid** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**promotions** | Option<[**Vec<String>**](String.md)> |  |  |
**content_manifest_ids** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::RealmsBasicPromoteRealmResponse**](RealmsBasicPromoteRealmResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_realms_promotion_post

> models::RealmsBasicPromoteRealmResponse basic_realms_promotion_post(x_beam_gamertag, realms_basic_promote_realm_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**realms_basic_promote_realm_request** | Option<[**RealmsBasicPromoteRealmRequest**](RealmsBasicPromoteRealmRequest.md)> |  |  |

### Return type

[**models::RealmsBasicPromoteRealmResponse**](RealmsBasicPromoteRealmResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

