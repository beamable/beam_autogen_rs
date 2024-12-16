# \AuthApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_auth_refresh_token_post**](AuthApi.md#api_auth_refresh_token_post) | **POST** /api/auth/refresh-token | 
[**api_auth_server_post**](AuthApi.md#api_auth_server_post) | **POST** /api/auth/server | 
[**api_auth_tokens_guest_post**](AuthApi.md#api_auth_tokens_guest_post) | **POST** /api/auth/tokens/guest | 
[**api_auth_tokens_password_post**](AuthApi.md#api_auth_tokens_password_post) | **POST** /api/auth/tokens/password | 
[**api_auth_tokens_refresh_token_post**](AuthApi.md#api_auth_tokens_refresh_token_post) | **POST** /api/auth/tokens/refresh-token | 



## api_auth_refresh_token_post

> models::AuthResponse api_auth_refresh_token_post(refresh_token_auth_request)


Generate a new access token for previously authenticated account. DEPRECATED: Use `tokens/refresh-token` instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_token_auth_request** | Option<[**RefreshTokenAuthRequest**](RefreshTokenAuthRequest.md)> | `RefreshTokenAuthRequest` |  |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_server_post

> models::ServerTokenResponse api_auth_server_post(server_token_auth_request)


Generate a new access token for a machine with a shared secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_token_auth_request** | Option<[**ServerTokenAuthRequest**](ServerTokenAuthRequest.md)> | `ServerTokenAuthRequest` |  |

### Return type

[**models::ServerTokenResponse**](ServerTokenResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_guest_post

> models::AuthResponse api_auth_tokens_guest_post(guest_auth_request)


Generate a new access token for a brand-new player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guest_auth_request** | Option<[**GuestAuthRequest**](GuestAuthRequest.md)> | `GuestAuthRequest` |  |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_password_post

> models::AuthResponse api_auth_tokens_password_post(password_auth_request)


Generate a new access token when given email and password credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_auth_request** | Option<[**PasswordAuthRequest**](PasswordAuthRequest.md)> | `PasswordAuthRequest` |  |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_refresh_token_post

> models::AuthResponse api_auth_tokens_refresh_token_post(refresh_token_auth_request)


Generate a new access token for previously authenticated account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_token_auth_request** | Option<[**RefreshTokenAuthRequest**](RefreshTokenAuthRequest.md)> | `RefreshTokenAuthRequest` |  |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

