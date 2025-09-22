# \CustomerApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_customers_activate_put**](CustomerApi.md#api_customers_activate_put) | **PUT** /api/customers/activate | 
[**api_customers_aliases_alias_get**](CustomerApi.md#api_customers_aliases_alias_get) | **GET** /api/customers/aliases/{alias} | 
[**api_customers_customer_id_alias_put**](CustomerApi.md#api_customers_customer_id_alias_put) | **PUT** /api/customers/{customerId}/alias | 
[**api_customers_customer_id_games_game_id_get**](CustomerApi.md#api_customers_customer_id_games_game_id_get) | **GET** /api/customers/{customerId}/games/{gameId} | 
[**api_customers_customer_id_games_game_id_put**](CustomerApi.md#api_customers_customer_id_games_game_id_put) | **PUT** /api/customers/{customerId}/games/{gameId} | 
[**api_customers_customer_id_games_get**](CustomerApi.md#api_customers_customer_id_games_get) | **GET** /api/customers/{customerId}/games | 
[**api_customers_customer_id_games_post**](CustomerApi.md#api_customers_customer_id_games_post) | **POST** /api/customers/{customerId}/games | 
[**api_customers_customer_id_get**](CustomerApi.md#api_customers_customer_id_get) | **GET** /api/customers/{customerId} | 
[**api_customers_customer_id_put**](CustomerApi.md#api_customers_customer_id_put) | **PUT** /api/customers/{customerId} | 
[**api_customers_customer_id_realms_destination_realm_id_promotion_get**](CustomerApi.md#api_customers_customer_id_realms_destination_realm_id_promotion_get) | **GET** /api/customers/{customerId}/realms/{destinationRealmId}/promotion | 
[**api_customers_customer_id_realms_destination_realm_id_promotion_post**](CustomerApi.md#api_customers_customer_id_realms_destination_realm_id_promotion_post) | **POST** /api/customers/{customerId}/realms/{destinationRealmId}/promotion | 
[**api_customers_customer_id_realms_post**](CustomerApi.md#api_customers_customer_id_realms_post) | **POST** /api/customers/{customerId}/realms | 
[**api_customers_customer_id_realms_realm_id_client_defaults_get**](CustomerApi.md#api_customers_customer_id_realms_realm_id_client_defaults_get) | **GET** /api/customers/{customerId}/realms/{realmId}/client-defaults | 
[**api_customers_customer_id_realms_realm_id_config_get**](CustomerApi.md#api_customers_customer_id_realms_realm_id_config_get) | **GET** /api/customers/{customerId}/realms/{realmId}/config | 
[**api_customers_customer_id_realms_realm_id_config_patch**](CustomerApi.md#api_customers_customer_id_realms_realm_id_config_patch) | **PATCH** /api/customers/{customerId}/realms/{realmId}/config | 
[**api_customers_customer_id_realms_realm_id_config_put**](CustomerApi.md#api_customers_customer_id_realms_realm_id_config_put) | **PUT** /api/customers/{customerId}/realms/{realmId}/config | 
[**api_customers_customer_id_realms_realm_id_delete**](CustomerApi.md#api_customers_customer_id_realms_realm_id_delete) | **DELETE** /api/customers/{customerId}/realms/{realmId} | 
[**api_customers_customer_id_realms_realm_id_get**](CustomerApi.md#api_customers_customer_id_realms_realm_id_get) | **GET** /api/customers/{customerId}/realms/{realmId} | 
[**api_customers_customer_id_realms_realm_id_put**](CustomerApi.md#api_customers_customer_id_realms_realm_id_put) | **PUT** /api/customers/{customerId}/realms/{realmId} | 
[**api_customers_customer_id_realms_realm_id_rename_put**](CustomerApi.md#api_customers_customer_id_realms_realm_id_rename_put) | **PUT** /api/customers/{customerId}/realms/{realmId}/rename | 
[**api_customers_get**](CustomerApi.md#api_customers_get) | **GET** /api/customers | 
[**api_customers_post**](CustomerApi.md#api_customers_post) | **POST** /api/customers | 
[**api_customers_verify_post**](CustomerApi.md#api_customers_verify_post) | **POST** /api/customers/verify | 



## api_customers_activate_put

> models::CustomerActorHtmlResponse api_customers_activate_put(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::CustomerActorHtmlResponse**](CustomerActorHtmlResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_aliases_alias_get

> models::CustomerActorAliasAvailableResponse api_customers_aliases_alias_get(alias, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::CustomerActorAliasAvailableResponse**](CustomerActorAliasAvailableResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_alias_put

> serde_json::Value api_customers_customer_id_alias_put(customer_id, x_beam_scope, x_beam_gamertag, set_customer_alias_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**set_customer_alias_request** | Option<[**SetCustomerAliasRequest**](SetCustomerAliasRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_games_game_id_get

> models::GetGamesResponse api_customers_customer_id_games_game_id_get(customer_id, game_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**game_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::GetGamesResponse**](GetGamesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_games_game_id_put

> serde_json::Value api_customers_customer_id_games_game_id_put(customer_id, game_id, x_beam_scope, x_beam_gamertag, customer_actor_update_game_hierarchy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**game_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**customer_actor_update_game_hierarchy_request** | Option<[**CustomerActorUpdateGameHierarchyRequest**](CustomerActorUpdateGameHierarchyRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_games_get

> models::GetGamesResponse api_customers_customer_id_games_get(customer_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::GetGamesResponse**](GetGamesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_games_post

> models::RealmView api_customers_customer_id_games_post(customer_id, x_beam_scope, x_beam_gamertag, new_game_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**new_game_request** | Option<[**NewGameRequest**](NewGameRequest.md)> |  |  |

### Return type

[**models::RealmView**](RealmView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_get

> models::CustomerActorCustomerView api_customers_customer_id_get(customer_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::CustomerActorCustomerView**](CustomerActorCustomerView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_put

> serde_json::Value api_customers_customer_id_put(customer_id, x_beam_scope, x_beam_gamertag, update_customer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**update_customer_request** | Option<[**UpdateCustomerRequest**](UpdateCustomerRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_destination_realm_id_promotion_get

> models::CustomerActorPromoteRealmResponse api_customers_customer_id_realms_destination_realm_id_promotion_get(customer_id, destination_realm_id, x_beam_scope, x_beam_gamertag, source_realm_id, promotables, content_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**destination_realm_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**source_realm_id** | Option<**String**> |  |  |
**promotables** | Option<**String**> |  |  |
**content_ids** | Option<**String**> |  |  |

### Return type

[**models::CustomerActorPromoteRealmResponse**](CustomerActorPromoteRealmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_destination_realm_id_promotion_post

> models::CustomerActorPromoteRealmResponse api_customers_customer_id_realms_destination_realm_id_promotion_post(customer_id, destination_realm_id, x_beam_scope, x_beam_gamertag, customer_actor_promote_realm_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**destination_realm_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**customer_actor_promote_realm_request** | Option<[**CustomerActorPromoteRealmRequest**](CustomerActorPromoteRealmRequest.md)> |  |  |

### Return type

[**models::CustomerActorPromoteRealmResponse**](CustomerActorPromoteRealmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_post

> serde_json::Value api_customers_customer_id_realms_post(customer_id, x_beam_scope, x_beam_gamertag, create_realm_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**create_realm_request** | Option<[**CreateRealmRequest**](CreateRealmRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_client_defaults_get

> models::CustomerActorRealmConfiguration api_customers_customer_id_realms_realm_id_client_defaults_get(customer_id, realm_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**realm_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::CustomerActorRealmConfiguration**](CustomerActorRealmConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_config_get

> models::CustomerActorRealmConfigResponse api_customers_customer_id_realms_realm_id_config_get(customer_id, realm_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**realm_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::CustomerActorRealmConfigResponse**](CustomerActorRealmConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_config_patch

> serde_json::Value api_customers_customer_id_realms_realm_id_config_patch(customer_id, realm_id, x_beam_scope, x_beam_gamertag, realm_config_change_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**realm_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**realm_config_change_request** | Option<[**RealmConfigChangeRequest**](RealmConfigChangeRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_config_put

> serde_json::Value api_customers_customer_id_realms_realm_id_config_put(customer_id, realm_id, x_beam_scope, x_beam_gamertag, customer_actor_realm_config_save_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**realm_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**customer_actor_realm_config_save_request** | Option<[**CustomerActorRealmConfigSaveRequest**](CustomerActorRealmConfigSaveRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_delete

> serde_json::Value api_customers_customer_id_realms_realm_id_delete(customer_id, realm_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**realm_id** | **String** |  | [required] |
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


## api_customers_customer_id_realms_realm_id_get

> models::RealmView api_customers_customer_id_realms_realm_id_get(customer_id, realm_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**realm_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::RealmView**](RealmView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_put

> serde_json::Value api_customers_customer_id_realms_realm_id_put(customer_id, realm_id, x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**realm_id** | **String** |  | [required] |
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


## api_customers_customer_id_realms_realm_id_rename_put

> serde_json::Value api_customers_customer_id_realms_realm_id_rename_put(customer_id, realm_id, x_beam_scope, x_beam_gamertag, rename_realm_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**realm_id** | **String** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**rename_realm_request** | Option<[**RenameRealmRequest**](RenameRealmRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_get

> models::CustomerActorCustomersResponse api_customers_get(x_beam_scope, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::CustomerActorCustomersResponse**](CustomerActorCustomersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_post

> models::CustomerActorNewCustomerResponse api_customers_post(x_beam_scope, x_beam_gamertag, customer_actor_new_customer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**customer_actor_new_customer_request** | Option<[**CustomerActorNewCustomerRequest**](CustomerActorNewCustomerRequest.md)> |  |  |

### Return type

[**models::CustomerActorNewCustomerResponse**](CustomerActorNewCustomerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_verify_post

> models::CustomerActorNewCustomerResponse api_customers_verify_post(x_beam_scope, x_beam_gamertag, customer_actor_new_customer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**customer_actor_new_customer_request** | Option<[**CustomerActorNewCustomerRequest**](CustomerActorNewCustomerRequest.md)> |  |  |

### Return type

[**models::CustomerActorNewCustomerResponse**](CustomerActorNewCustomerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

