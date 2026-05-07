# \AuthApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_auth_auth_codes_post**](AuthApi.md#api_auth_auth_codes_post) | **POST** /api/auth/auth-codes | 
[**api_auth_keys_get**](AuthApi.md#api_auth_keys_get) | **GET** /api/auth/keys | 
[**api_auth_refresh_token_post**](AuthApi.md#api_auth_refresh_token_post) | **POST** /api/auth/refresh-token | 
[**api_auth_server_post**](AuthApi.md#api_auth_server_post) | **POST** /api/auth/server | 
[**api_auth_tokens_access_token_id_get**](AuthApi.md#api_auth_tokens_access_token_id_get) | **GET** /api/auth/tokens/access/{tokenId} | 
[**api_auth_tokens_auth_code_post**](AuthApi.md#api_auth_tokens_auth_code_post) | **POST** /api/auth/tokens/auth-code | 
[**api_auth_tokens_delete**](AuthApi.md#api_auth_tokens_delete) | **DELETE** /api/auth/tokens | 
[**api_auth_tokens_device_id_post**](AuthApi.md#api_auth_tokens_device_id_post) | **POST** /api/auth/tokens/device-id | 
[**api_auth_tokens_external_post**](AuthApi.md#api_auth_tokens_external_post) | **POST** /api/auth/tokens/external | 
[**api_auth_tokens_get**](AuthApi.md#api_auth_tokens_get) | **GET** /api/auth/tokens | 
[**api_auth_tokens_guest_post**](AuthApi.md#api_auth_tokens_guest_post) | **POST** /api/auth/tokens/guest | 
[**api_auth_tokens_password_post**](AuthApi.md#api_auth_tokens_password_post) | **POST** /api/auth/tokens/password | 
[**api_auth_tokens_refresh_token_post**](AuthApi.md#api_auth_tokens_refresh_token_post) | **POST** /api/auth/tokens/refresh-token | 
[**api_auth_tokens_token_id_get**](AuthApi.md#api_auth_tokens_token_id_get) | **GET** /api/auth/tokens/{tokenId} | 
[**api_well_known_openid_configuration_get**](AuthApi.md#api_well_known_openid_configuration_get) | **GET** /api/.well-known/openid-configuration | 
[**basic_auth_token_get**](AuthApi.md#basic_auth_token_get) | **GET** /basic/auth/token | 
[**basic_auth_token_list_get**](AuthApi.md#basic_auth_token_list_get) | **GET** /basic/auth/token/list | 
[**basic_auth_token_post**](AuthApi.md#basic_auth_token_post) | **POST** /basic/auth/token | 
[**basic_auth_token_revoke_put**](AuthApi.md#basic_auth_token_revoke_put) | **PUT** /basic/auth/token/revoke | 



## api_auth_auth_codes_post

> models::AuthV2AuthCode api_auth_auth_codes_post(x_beam_gamertag, x_beam_timeout, auth_v2_auth_code_request)


[Internal] Generate an authorization code for OAuth 2.0 flow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_auth_code_request** | Option<[**AuthV2AuthCodeRequest**](AuthV2AuthCodeRequest.md)> | Authorization code grant request |  |

### Return type

[**models::AuthV2AuthCode**](AuthV2AuthCode.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_keys_get

> models::AuthV2JsonWebKeySet api_auth_keys_get(x_beam_gamertag, x_beam_timeout)


Returns the public keys used for JWT verification in JWKS format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::AuthV2JsonWebKeySet**](AuthV2JsonWebKeySet.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_refresh_token_post

> models::AuthV2AuthResponse api_auth_refresh_token_post(x_beam_gamertag, x_beam_timeout, auth_v2_refresh_token_auth_request)


Generate a new access token for a previously authenticated account. DEPRECATED: Use `tokens/refresh-token` instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_refresh_token_auth_request** | Option<[**AuthV2RefreshTokenAuthRequest**](AuthV2RefreshTokenAuthRequest.md)> | `RefreshTokenAuthRequest` |  |

### Return type

[**models::AuthV2AuthResponse**](AuthV2AuthResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_server_post

> models::AuthV2ServerTokenResponse api_auth_server_post(x_beam_gamertag, x_beam_timeout, auth_v2_server_token_auth_request)


Generate a new access token for a machine with a shared secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_server_token_auth_request** | Option<[**AuthV2ServerTokenAuthRequest**](AuthV2ServerTokenAuthRequest.md)> | `ServerTokenAuthRequest` |  |

### Return type

[**models::AuthV2ServerTokenResponse**](AuthV2ServerTokenResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_access_token_id_get

> models::AuthV2LegacyAccessToken api_auth_tokens_access_token_id_get(token_id, x_beam_gamertag, x_beam_timeout, customer_id, realm_id)


Validate an access token and retrieve its details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | The access token to validate | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**customer_id** | Option<**String**> | Customer ID |  |
**realm_id** | Option<**String**> | Realm ID |  |

### Return type

[**models::AuthV2LegacyAccessToken**](AuthV2LegacyAccessToken.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_auth_code_post

> models::AuthV2AuthResponse api_auth_tokens_auth_code_post(x_beam_gamertag, x_beam_timeout, auth_v2_authorization_code_auth_request)


Generate a new access token using an OAuth 2.0 authorization code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_authorization_code_auth_request** | Option<[**AuthV2AuthorizationCodeAuthRequest**](AuthV2AuthorizationCodeAuthRequest.md)> | Authorization code authentication request |  |

### Return type

[**models::AuthV2AuthResponse**](AuthV2AuthResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_delete

> serde_json::Value api_auth_tokens_delete(x_beam_gamertag, x_beam_timeout, auth_v2_revoke_refresh_tokens_request)


Revoke one or more refresh tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_revoke_refresh_tokens_request** | Option<[**AuthV2RevokeRefreshTokensRequest**](AuthV2RevokeRefreshTokensRequest.md)> | Request body with the token ids to revoke |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_device_id_post

> models::AuthV2AuthResponse api_auth_tokens_device_id_post(x_beam_gamertag, x_beam_timeout, auth_v2_device_id_auth_request)


Generate a new access token using a device identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_device_id_auth_request** | Option<[**AuthV2DeviceIdAuthRequest**](AuthV2DeviceIdAuthRequest.md)> | Device ID authentication request |  |

### Return type

[**models::AuthV2AuthResponse**](AuthV2AuthResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_external_post

> models::AuthV2ExternalAuthResponse api_auth_tokens_external_post(x_beam_gamertag, x_beam_timeout, auth_v2_external_auth_request)


Generate a new access token using external OAuth credentials. These include Facebook, Google, etc. For external providers we haven't implemented, we're going to make a federated authentication request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_external_auth_request** | Option<[**AuthV2ExternalAuthRequest**](AuthV2ExternalAuthRequest.md)> | External authentication request containing provider and token |  |

### Return type

[**models::AuthV2ExternalAuthResponse**](AuthV2ExternalAuthResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_get

> models::AuthV2ListTokensResponse api_auth_tokens_get(x_beam_gamertag, x_beam_timeout, player_id_or_account_id, skip, limit)


List all active refresh tokens for an account or player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**player_id_or_account_id** | Option<**String**> | The gamer tag or account ID to list tokens for |  |
**skip** | Option<**i32**> | Skips N items |  |[default to 0]
**limit** | Option<**i32**> | Max number of items to return |  |[default to 100]

### Return type

[**models::AuthV2ListTokensResponse**](AuthV2ListTokensResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_guest_post

> models::AuthV2AuthResponse api_auth_tokens_guest_post(x_beam_gamertag, x_beam_timeout, auth_v2_guest_auth_request)


Generate a new access token for a brand-new player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_guest_auth_request** | Option<[**AuthV2GuestAuthRequest**](AuthV2GuestAuthRequest.md)> | `GuestAuthRequest` |  |

### Return type

[**models::AuthV2AuthResponse**](AuthV2AuthResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_password_post

> models::AuthV2AuthResponse api_auth_tokens_password_post(x_beam_gamertag, x_beam_timeout, auth_v2_password_auth_request)


Generate a new access token when given email and password credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_password_auth_request** | Option<[**AuthV2PasswordAuthRequest**](AuthV2PasswordAuthRequest.md)> | `PasswordAuthRequest` |  |

### Return type

[**models::AuthV2AuthResponse**](AuthV2AuthResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_refresh_token_post

> models::AuthV2AuthResponse api_auth_tokens_refresh_token_post(x_beam_gamertag, x_beam_timeout, auth_v2_refresh_token_auth_request)


Generate a new access token for a previously authenticated account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**auth_v2_refresh_token_auth_request** | Option<[**AuthV2RefreshTokenAuthRequest**](AuthV2RefreshTokenAuthRequest.md)> | `RefreshTokenAuthRequest` |  |

### Return type

[**models::AuthV2AuthResponse**](AuthV2AuthResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_tokens_token_id_get

> models::AuthV2RefreshToken api_auth_tokens_token_id_get(token_id, x_beam_gamertag, x_beam_timeout)


Retrieve full details of a refresh token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | The refresh token id to look up | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::AuthV2RefreshToken**](AuthV2RefreshToken.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_well_known_openid_configuration_get

> models::AuthV2OpenIdConfigResponse api_well_known_openid_configuration_get(x_beam_gamertag, x_beam_timeout)


Returns the OpenID Connect discovery document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::AuthV2OpenIdConfigResponse**](AuthV2OpenIdConfigResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_auth_token_get

> models::Token basic_auth_token_get(token, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::Token**](Token.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_auth_token_list_get

> models::ListTokenResponse basic_auth_token_list_get(page_size, page, gamer_tag_or_account_id, x_beam_gamertag, cid, pid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | **i32** |  | [required] |
**page** | **i32** |  | [required] |
**gamer_tag_or_account_id** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**cid** | Option<**i64**> |  |  |
**pid** | Option<**String**> |  |  |

### Return type

[**models::ListTokenResponse**](ListTokenResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_auth_token_post

> models::TokenResponse basic_auth_token_post(x_beam_gamertag, token_request_wrapper)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**token_request_wrapper** | Option<[**TokenRequestWrapper**](TokenRequestWrapper.md)> |  |  |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_auth_token_revoke_put

> models::CommonResponse basic_auth_token_revoke_put(x_beam_gamertag, revoke_token_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**revoke_token_request** | Option<[**RevokeTokenRequest**](RevokeTokenRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

