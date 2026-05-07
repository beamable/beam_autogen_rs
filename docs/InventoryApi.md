# \InventoryApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_inventory_currency_get**](InventoryApi.md#basic_inventory_currency_get) | **GET** /basic/inventory/currency | 
[**basic_inventory_items_get**](InventoryApi.md#basic_inventory_items_get) | **GET** /basic/inventory/items | 
[**object_inventory_object_id_get**](InventoryApi.md#object_inventory_object_id_get) | **GET** /object/inventory/{objectId}/ | 
[**object_inventory_object_id_multipliers_get**](InventoryApi.md#object_inventory_object_id_multipliers_get) | **GET** /object/inventory/{objectId}/multipliers | 
[**object_inventory_object_id_post**](InventoryApi.md#object_inventory_object_id_post) | **POST** /object/inventory/{objectId}/ | 
[**object_inventory_object_id_preview_put**](InventoryApi.md#object_inventory_object_id_preview_put) | **PUT** /object/inventory/{objectId}/preview | 
[**object_inventory_object_id_proxy_reload_put**](InventoryApi.md#object_inventory_object_id_proxy_reload_put) | **PUT** /object/inventory/{objectId}/proxy/reload | 
[**object_inventory_object_id_put**](InventoryApi.md#object_inventory_object_id_put) | **PUT** /object/inventory/{objectId}/ | 
[**object_inventory_object_id_transaction_delete**](InventoryApi.md#object_inventory_object_id_transaction_delete) | **DELETE** /object/inventory/{objectId}/transaction | 
[**object_inventory_object_id_transfer_put**](InventoryApi.md#object_inventory_object_id_transfer_put) | **PUT** /object/inventory/{objectId}/transfer | 



## basic_inventory_currency_get

> models::CurrencyContentResponse basic_inventory_currency_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CurrencyContentResponse**](CurrencyContentResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_inventory_items_get

> models::ItemContentResponse basic_inventory_items_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ItemContentResponse**](ItemContentResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_get

> models::InventoryView object_inventory_object_id_get(object_id, x_beam_gamertag, scope)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**scope** | Option<**String**> |  |  |

### Return type

[**models::InventoryView**](InventoryView.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_multipliers_get

> models::MultipliersGetResponse object_inventory_object_id_multipliers_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::MultipliersGetResponse**](MultipliersGetResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_post

> models::InventoryView object_inventory_object_id_post(object_id, x_beam_gamertag, inventory_query_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**inventory_query_request** | Option<[**InventoryQueryRequest**](InventoryQueryRequest.md)> |  |  |

### Return type

[**models::InventoryView**](InventoryView.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_preview_put

> models::PreviewVipBonusResponse object_inventory_object_id_preview_put(object_id, x_beam_gamertag, inventory_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**inventory_update_request** | Option<[**InventoryUpdateRequest**](InventoryUpdateRequest.md)> |  |  |

### Return type

[**models::PreviewVipBonusResponse**](PreviewVipBonusResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_proxy_reload_put

> models::CommonResponse object_inventory_object_id_proxy_reload_put(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_put

> models::InventoryUpdateResponse object_inventory_object_id_put(object_id, x_beam_gamertag, inventory_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**inventory_update_request** | Option<[**InventoryUpdateRequest**](InventoryUpdateRequest.md)> |  |  |

### Return type

[**models::InventoryUpdateResponse**](InventoryUpdateResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_transaction_delete

> models::CommonResponse object_inventory_object_id_transaction_delete(object_id, x_beam_gamertag, end_transaction_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**end_transaction_request** | Option<[**EndTransactionRequest**](EndTransactionRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_transfer_put

> models::CommonResponse object_inventory_object_id_transfer_put(object_id, x_beam_gamertag, transfer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**transfer_request** | Option<[**TransferRequest**](TransferRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

