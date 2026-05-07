# \CommerceApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_commerce_catalog_get**](CommerceApi.md#basic_commerce_catalog_get) | **GET** /basic/commerce/catalog | 
[**basic_commerce_catalog_legacy_post**](CommerceApi.md#basic_commerce_catalog_legacy_post) | **POST** /basic/commerce/catalog/legacy | 
[**basic_commerce_skus_get**](CommerceApi.md#basic_commerce_skus_get) | **GET** /basic/commerce/skus | 
[**basic_commerce_skus_post**](CommerceApi.md#basic_commerce_skus_post) | **POST** /basic/commerce/skus | 
[**object_commerce_object_id_coupons_count_get**](CommerceApi.md#object_commerce_object_id_coupons_count_get) | **GET** /object/commerce/{objectId}/coupons/count | 
[**object_commerce_object_id_coupons_post**](CommerceApi.md#object_commerce_object_id_coupons_post) | **POST** /object/commerce/{objectId}/coupons | 
[**object_commerce_object_id_get**](CommerceApi.md#object_commerce_object_id_get) | **GET** /object/commerce/{objectId}/ | 
[**object_commerce_object_id_listings_cooldown_put**](CommerceApi.md#object_commerce_object_id_listings_cooldown_put) | **PUT** /object/commerce/{objectId}/listings/cooldown | 
[**object_commerce_object_id_listings_get**](CommerceApi.md#object_commerce_object_id_listings_get) | **GET** /object/commerce/{objectId}/listings | 
[**object_commerce_object_id_offers_admin_get**](CommerceApi.md#object_commerce_object_id_offers_admin_get) | **GET** /object/commerce/{objectId}/offersAdmin | 
[**object_commerce_object_id_offers_get**](CommerceApi.md#object_commerce_object_id_offers_get) | **GET** /object/commerce/{objectId}/offers | 
[**object_commerce_object_id_purchase_post**](CommerceApi.md#object_commerce_object_id_purchase_post) | **POST** /object/commerce/{objectId}/purchase | 
[**object_commerce_object_id_purchase_put**](CommerceApi.md#object_commerce_object_id_purchase_put) | **PUT** /object/commerce/{objectId}/purchase | 
[**object_commerce_object_id_stats_update_post**](CommerceApi.md#object_commerce_object_id_stats_update_post) | **POST** /object/commerce/{objectId}/stats/update | 
[**object_commerce_object_id_status_delete**](CommerceApi.md#object_commerce_object_id_status_delete) | **DELETE** /object/commerce/{objectId}/status | 



## basic_commerce_catalog_get

> models::GetCatalogResponse basic_commerce_catalog_get(x_beam_gamertag, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**version** | Option<**i64**> |  |  |

### Return type

[**models::GetCatalogResponse**](GetCatalogResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_commerce_catalog_legacy_post

> models::ResultResponse basic_commerce_catalog_legacy_post(x_beam_gamertag, save_catalog_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**save_catalog_request** | Option<[**SaveCatalogRequest**](SaveCatalogRequest.md)> |  |  |

### Return type

[**models::ResultResponse**](ResultResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_commerce_skus_get

> models::GetSkusResponse basic_commerce_skus_get(x_beam_gamertag, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**version** | Option<**i64**> |  |  |

### Return type

[**models::GetSkusResponse**](GetSKUsResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_commerce_skus_post

> models::ResultResponse basic_commerce_skus_post(x_beam_gamertag, save_skus_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**save_skus_request** | Option<[**SaveSkusRequest**](SaveSkusRequest.md)> |  |  |

### Return type

[**models::ResultResponse**](ResultResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_coupons_count_get

> models::GetTotalCouponResponse object_commerce_object_id_coupons_count_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetTotalCouponResponse**](GetTotalCouponResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_coupons_post

> models::CommonResponse object_commerce_object_id_coupons_post(object_id, x_beam_gamertag, give_coupon_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**give_coupon_req** | Option<[**GiveCouponReq**](GiveCouponReq.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_get

> models::GetActiveOffersResponse object_commerce_object_id_get(object_id, x_beam_gamertag, scope)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**scope** | Option<**String**> |  |  |

### Return type

[**models::GetActiveOffersResponse**](GetActiveOffersResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_listings_cooldown_put

> models::CommonResponse object_commerce_object_id_listings_cooldown_put(object_id, x_beam_gamertag, cooldown_modifier_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**cooldown_modifier_request** | Option<[**CooldownModifierRequest**](CooldownModifierRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_listings_get

> models::ActiveListingResponse object_commerce_object_id_listings_get(listing, object_id, x_beam_gamertag, store, time)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listing** | **String** |  | [required] |
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**store** | Option<**String**> |  |  |
**time** | Option<**String**> |  |  |

### Return type

[**models::ActiveListingResponse**](ActiveListingResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_offers_admin_get

> models::GetActiveOffersResponse object_commerce_object_id_offers_admin_get(object_id, x_beam_gamertag, language, time, stores)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**language** | Option<**String**> |  |  |
**time** | Option<**String**> |  |  |
**stores** | Option<**String**> |  |  |

### Return type

[**models::GetActiveOffersResponse**](GetActiveOffersResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_offers_get

> models::GetActiveOffersResponse object_commerce_object_id_offers_get(object_id, x_beam_gamertag, language, time, stores)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**language** | Option<**String**> |  |  |
**time** | Option<**String**> |  |  |
**stores** | Option<**String**> |  |  |

### Return type

[**models::GetActiveOffersResponse**](GetActiveOffersResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_purchase_post

> models::InventoryUpdateResponse object_commerce_object_id_purchase_post(object_id, x_beam_gamertag, purchase_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**purchase_request** | Option<[**PurchaseRequest**](PurchaseRequest.md)> |  |  |

### Return type

[**models::InventoryUpdateResponse**](InventoryUpdateResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_purchase_put

> models::ResultResponse object_commerce_object_id_purchase_put(object_id, x_beam_gamertag, report_purchase_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**report_purchase_request** | Option<[**ReportPurchaseRequest**](ReportPurchaseRequest.md)> |  |  |

### Return type

[**models::ResultResponse**](ResultResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_stats_update_post

> models::CommonResponse object_commerce_object_id_stats_update_post(object_id, x_beam_gamertag, stat_subscription_notification)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**stat_subscription_notification** | Option<[**StatSubscriptionNotification**](StatSubscriptionNotification.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_commerce_object_id_status_delete

> models::CommonResponse object_commerce_object_id_status_delete(object_id, x_beam_gamertag, clear_status_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**clear_status_request** | Option<[**ClearStatusRequest**](ClearStatusRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

