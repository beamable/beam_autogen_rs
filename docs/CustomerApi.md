# \CustomerApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_customers_activate_put**](CustomerApi.md#api_customers_activate_put) | **PUT** /api/customers/activate | 
[**api_customers_aliases_alias_get**](CustomerApi.md#api_customers_aliases_alias_get) | **GET** /api/customers/aliases/{alias} | 
[**api_customers_customer_id_admin_view_get**](CustomerApi.md#api_customers_customer_id_admin_view_get) | **GET** /api/customers/{customerId}/admin-view | 
[**api_customers_customer_id_config_get**](CustomerApi.md#api_customers_customer_id_config_get) | **GET** /api/customers/{customerId}/config | 
[**api_customers_customer_id_games_game_id_get**](CustomerApi.md#api_customers_customer_id_games_game_id_get) | **GET** /api/customers/{customerId}/games/{gameId} | 
[**api_customers_customer_id_games_game_id_put**](CustomerApi.md#api_customers_customer_id_games_game_id_put) | **PUT** /api/customers/{customerId}/games/{gameId} | 
[**api_customers_customer_id_games_get**](CustomerApi.md#api_customers_customer_id_games_get) | **GET** /api/customers/{customerId}/games | 
[**api_customers_customer_id_games_post**](CustomerApi.md#api_customers_customer_id_games_post) | **POST** /api/customers/{customerId}/games | 
[**api_customers_customer_id_get**](CustomerApi.md#api_customers_customer_id_get) | **GET** /api/customers/{customerId} | 
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
[**api_customers_customer_id_stripe_subscription_get**](CustomerApi.md#api_customers_customer_id_stripe_subscription_get) | **GET** /api/customers/{customerId}/stripe/subscription | 
[**api_customers_get**](CustomerApi.md#api_customers_get) | **GET** /api/customers | 
[**api_customers_post**](CustomerApi.md#api_customers_post) | **POST** /api/customers | 
[**api_customers_verify_post**](CustomerApi.md#api_customers_verify_post) | **POST** /api/customers/verify | 



## api_customers_activate_put

> serde_json::Value api_customers_activate_put(x_beam_gamertag, x_beam_timeout)


Activate a pending customer account. Returns an HTML redirect page to the portal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## api_customers_aliases_alias_get

> models::CustomerActorAliasAvailableResponse api_customers_aliases_alias_get(alias, x_beam_gamertag, x_beam_timeout)


Check whether a customer alias is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | The alias to check. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::CustomerActorAliasAvailableResponse**](CustomerActorAliasAvailableResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_admin_view_get

> models::CustomerActorCustomer api_customers_customer_id_admin_view_get(customer_id, x_beam_gamertag, x_beam_timeout, show_hidden_realms)


Get the full admin view of a customer (includes all fields).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customer ID to look up. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**show_hidden_realms** | Option<**bool**> | Whether to include hidden realms in the response. |  |[default to false]

### Return type

[**models::CustomerActorCustomer**](CustomerActorCustomer.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_config_get

> models::CustomerActorRealmConfigResponse api_customers_customer_id_config_get(customer_id, x_beam_gamertag, x_beam_timeout)


Get the customer-level realm configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer to retrieve config for. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::CustomerActorRealmConfigResponse**](CustomerActorRealmConfigResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_games_game_id_get

> models::GetGamesResponse api_customers_customer_id_games_game_id_get(customer_id, game_id, x_beam_gamertag, x_beam_timeout, show_hidden_realms)


Get all realms under a specific game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**game_id** | **String** | ID of the game realm to retrieve realms for. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**show_hidden_realms** | Option<**bool**> | Whether to include hidden realms. |  |[default to false]

### Return type

[**models::GetGamesResponse**](GetGamesResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_games_game_id_put

> serde_json::Value api_customers_customer_id_games_game_id_put(customer_id, game_id, x_beam_gamertag, x_beam_timeout, customer_actor_update_game_hierarchy_request)


Update the realm hierarchy for a game (add/remove/reorder child realms).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**game_id** | **String** | ID of the game realm to update. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**customer_actor_update_game_hierarchy_request** | Option<[**CustomerActorUpdateGameHierarchyRequest**](CustomerActorUpdateGameHierarchyRequest.md)> | Updated realm hierarchy. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_games_get

> models::GetGamesResponse api_customers_customer_id_games_get(customer_id, x_beam_gamertag, x_beam_timeout, show_hidden_realms)


List all games (top-level realms) for a customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**show_hidden_realms** | Option<**bool**> | Whether to include hidden realms. |  |[default to false]

### Return type

[**models::GetGamesResponse**](GetGamesResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_games_post

> models::RealmView api_customers_customer_id_games_post(customer_id, x_beam_gamertag, x_beam_timeout, customer_actor_new_game_request)


Create a new game (top-level realm) under a customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer to create the game under. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**customer_actor_new_game_request** | Option<[**CustomerActorNewGameRequest**](CustomerActorNewGameRequest.md)> | Game creation request. |  |

### Return type

[**models::RealmView**](RealmView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_get

> models::CustomerActorCustomerView api_customers_customer_id_get(customer_id, x_beam_gamertag, x_beam_timeout, show_hidden_realms)


Get a specific customer by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customer ID to look up. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**show_hidden_realms** | Option<**bool**> | Whether to include hidden realms in the response. |  |[default to false]

### Return type

[**models::CustomerActorCustomerView**](CustomerActorCustomerView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_destination_realm_id_promotion_get

> models::CustomerActorPromoteRealmResponse api_customers_customer_id_realms_destination_realm_id_promotion_get(customer_id, destination_realm_id, x_beam_gamertag, x_beam_timeout, source_realm_id, promotables, content_ids)


Preview what would be promoted from one realm to another without applying changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**destination_realm_id** | **String** | ID of the destination realm. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**source_realm_id** | Option<**String**> | ID of the source realm. |  |
**promotables** | Option<**String**> | Comma-separated list of promotable types to include. |  |
**content_ids** | Option<**String**> | Comma-separated list of content IDs to filter by. |  |

### Return type

[**models::CustomerActorPromoteRealmResponse**](CustomerActorPromoteRealmResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_destination_realm_id_promotion_post

> models::CustomerActorPromoteRealmResponse api_customers_customer_id_realms_destination_realm_id_promotion_post(customer_id, destination_realm_id, x_beam_gamertag, x_beam_timeout, customer_actor_promote_realm_request)


Promote realm configuration and content from a source realm to a destination realm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**destination_realm_id** | **String** | ID of the realm to promote content into. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**customer_actor_promote_realm_request** | Option<[**CustomerActorPromoteRealmRequest**](CustomerActorPromoteRealmRequest.md)> | Promotion request specifying source realm and what to promote. |  |

### Return type

[**models::CustomerActorPromoteRealmResponse**](CustomerActorPromoteRealmResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_post

> serde_json::Value api_customers_customer_id_realms_post(customer_id, x_beam_gamertag, x_beam_timeout, create_realm_request)


Create a new realm under a customer's game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer to create the realm under. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**create_realm_request** | Option<[**CreateRealmRequest**](CreateRealmRequest.md)> | Realm creation request. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_client_defaults_get

> models::CustomerActorRealmConfiguration api_customers_customer_id_realms_realm_id_client_defaults_get(customer_id, realm_id, x_beam_gamertag, x_beam_timeout)


Get the client-facing realm configuration defaults.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**realm_id** | **String** | ID of the realm. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::CustomerActorRealmConfiguration**](CustomerActorRealmConfiguration.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_config_get

> models::CustomerActorRealmConfigResponse api_customers_customer_id_realms_realm_id_config_get(customer_id, realm_id, x_beam_gamertag, x_beam_timeout)


Get the configuration for a specific realm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**realm_id** | **String** | ID of the realm to retrieve config for. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::CustomerActorRealmConfigResponse**](CustomerActorRealmConfigResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_config_patch

> serde_json::Value api_customers_customer_id_realms_realm_id_config_patch(customer_id, realm_id, x_beam_gamertag, x_beam_timeout, realm_config_change_request)


Apply incremental changes to a realm's configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**realm_id** | **String** | ID of the realm to update. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**realm_config_change_request** | Option<[**RealmConfigChangeRequest**](RealmConfigChangeRequest.md)> | Config change request with keys to add, update, or remove. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_config_put

> serde_json::Value api_customers_customer_id_realms_realm_id_config_put(customer_id, realm_id, x_beam_gamertag, x_beam_timeout, customer_actor_realm_config_save_request)


Replace the entire configuration of a realm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**realm_id** | **String** | ID of the realm to update. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**customer_actor_realm_config_save_request** | Option<[**CustomerActorRealmConfigSaveRequest**](CustomerActorRealmConfigSaveRequest.md)> | Replacement configuration values. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_delete

> serde_json::Value api_customers_customer_id_realms_realm_id_delete(customer_id, realm_id, x_beam_gamertag, x_beam_timeout)


Archive (soft-delete) a realm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer that owns the realm. | [required] |
**realm_id** | **String** | ID of the realm to archive. | [required] |
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


## api_customers_customer_id_realms_realm_id_get

> models::RealmView api_customers_customer_id_realms_realm_id_get(customer_id, realm_id, x_beam_gamertag, x_beam_timeout)


Get a specific realm by customer and realm ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**realm_id** | **String** | ID of the realm to retrieve. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::RealmView**](RealmView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_put

> serde_json::Value api_customers_customer_id_realms_realm_id_put(customer_id, realm_id, x_beam_gamertag, x_beam_timeout, update_realm_request)


Update the hidden or archive status of a realm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer that owns the realm. | [required] |
**realm_id** | **String** | ID of the realm to update. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**update_realm_request** | Option<[**UpdateRealmRequest**](UpdateRealmRequest.md)> | Realm status update request. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_realms_realm_id_rename_put

> serde_json::Value api_customers_customer_id_realms_realm_id_rename_put(customer_id, realm_id, x_beam_gamertag, x_beam_timeout, rename_realm_request)


Rename a realm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer that owns the realm. | [required] |
**realm_id** | **String** | ID of the realm to rename. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**rename_realm_request** | Option<[**RenameRealmRequest**](RenameRealmRequest.md)> | Rename request with the new name. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_customer_id_stripe_subscription_get

> models::StripeSubscriptionResponse api_customers_customer_id_stripe_subscription_get(customer_id, x_beam_gamertag, x_beam_timeout)


Get the current Stripe subscription tier for a customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::StripeSubscriptionResponse**](StripeSubscriptionResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_get

> models::CustomerActorCustomersResponse api_customers_get(x_beam_gamertag, x_beam_timeout, show_hidden_realms)


List all customers visible to the requesting user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**show_hidden_realms** | Option<**bool**> | Whether to include hidden realms in the response. |  |[default to false]

### Return type

[**models::CustomerActorCustomersResponse**](CustomerActorCustomersResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_post

> models::CustomerActorNewCustomerResponse api_customers_post(x_beam_gamertag, x_beam_timeout, customer_actor_new_customer_request)


Create a new customer (org) and automatically activate it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**customer_actor_new_customer_request** | Option<[**CustomerActorNewCustomerRequest**](CustomerActorNewCustomerRequest.md)> | Customer creation request. |  |

### Return type

[**models::CustomerActorNewCustomerResponse**](CustomerActorNewCustomerResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_customers_verify_post

> models::CustomerActorNewCustomerResponse api_customers_verify_post(x_beam_gamertag, x_beam_timeout, customer_actor_new_customer_request)


Create a new customer and hold it in a pending state until email verification is complete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**customer_actor_new_customer_request** | Option<[**CustomerActorNewCustomerRequest**](CustomerActorNewCustomerRequest.md)> | Customer creation request. |  |

### Return type

[**models::CustomerActorNewCustomerResponse**](CustomerActorNewCustomerResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

