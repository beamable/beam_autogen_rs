# \AccountsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_accounts_admin_admin_user_post**](AccountsApi.md#basic_accounts_admin_admin_user_post) | **POST** /basic/accounts/admin/admin-user | 
[**basic_accounts_admin_admin_users_get**](AccountsApi.md#basic_accounts_admin_admin_users_get) | **GET** /basic/accounts/admin/admin-users | 
[**basic_accounts_admin_me_get**](AccountsApi.md#basic_accounts_admin_me_get) | **GET** /basic/accounts/admin/me | 
[**basic_accounts_available_device_id_get**](AccountsApi.md#basic_accounts_available_device_id_get) | **GET** /basic/accounts/available/device-id | 
[**basic_accounts_available_external_identity_get**](AccountsApi.md#basic_accounts_available_external_identity_get) | **GET** /basic/accounts/available/external_identity | 
[**basic_accounts_available_get**](AccountsApi.md#basic_accounts_available_get) | **GET** /basic/accounts/available | 
[**basic_accounts_available_third_party_get**](AccountsApi.md#basic_accounts_available_third_party_get) | **GET** /basic/accounts/available/third-party | 
[**basic_accounts_email_update_confirm_post**](AccountsApi.md#basic_accounts_email_update_confirm_post) | **POST** /basic/accounts/email-update/confirm | 
[**basic_accounts_email_update_init_post**](AccountsApi.md#basic_accounts_email_update_init_post) | **POST** /basic/accounts/email-update/init | 
[**basic_accounts_external_identity_delete**](AccountsApi.md#basic_accounts_external_identity_delete) | **DELETE** /basic/accounts/external_identity | 
[**basic_accounts_external_identity_post**](AccountsApi.md#basic_accounts_external_identity_post) | **POST** /basic/accounts/external_identity | 
[**basic_accounts_find_get**](AccountsApi.md#basic_accounts_find_get) | **GET** /basic/accounts/find | 
[**basic_accounts_get_personally_identifiable_information_get**](AccountsApi.md#basic_accounts_get_personally_identifiable_information_get) | **GET** /basic/accounts/get-personally-identifiable-information | 
[**basic_accounts_me_device_delete**](AccountsApi.md#basic_accounts_me_device_delete) | **DELETE** /basic/accounts/me/device | 
[**basic_accounts_me_get**](AccountsApi.md#basic_accounts_me_get) | **GET** /basic/accounts/me | 
[**basic_accounts_me_put**](AccountsApi.md#basic_accounts_me_put) | **PUT** /basic/accounts/me | 
[**basic_accounts_me_third_party_delete**](AccountsApi.md#basic_accounts_me_third_party_delete) | **DELETE** /basic/accounts/me/third-party | 
[**basic_accounts_password_update_confirm_post**](AccountsApi.md#basic_accounts_password_update_confirm_post) | **POST** /basic/accounts/password-update/confirm | 
[**basic_accounts_password_update_init_post**](AccountsApi.md#basic_accounts_password_update_init_post) | **POST** /basic/accounts/password-update/init | 
[**basic_accounts_register_post**](AccountsApi.md#basic_accounts_register_post) | **POST** /basic/accounts/register | 
[**basic_accounts_search_get**](AccountsApi.md#basic_accounts_search_get) | **GET** /basic/accounts/search | 
[**basic_accounts_signup_post**](AccountsApi.md#basic_accounts_signup_post) | **POST** /basic/accounts/signup | 
[**object_accounts_object_id_admin_email_put**](AccountsApi.md#object_accounts_object_id_admin_email_put) | **PUT** /object/accounts/{objectId}/admin/email | 
[**object_accounts_object_id_admin_forget_delete**](AccountsApi.md#object_accounts_object_id_admin_forget_delete) | **DELETE** /object/accounts/{objectId}/admin/forget | 
[**object_accounts_object_id_admin_scope_delete**](AccountsApi.md#object_accounts_object_id_admin_scope_delete) | **DELETE** /object/accounts/{objectId}/admin/scope | 
[**object_accounts_object_id_admin_scope_put**](AccountsApi.md#object_accounts_object_id_admin_scope_put) | **PUT** /object/accounts/{objectId}/admin/scope | 
[**object_accounts_object_id_admin_third_party_delete**](AccountsApi.md#object_accounts_object_id_admin_third_party_delete) | **DELETE** /object/accounts/{objectId}/admin/third-party | 
[**object_accounts_object_id_admin_third_party_put**](AccountsApi.md#object_accounts_object_id_admin_third_party_put) | **PUT** /object/accounts/{objectId}/admin/third-party | 
[**object_accounts_object_id_available_roles_get**](AccountsApi.md#object_accounts_object_id_available_roles_get) | **GET** /object/accounts/{objectId}/available-roles | 
[**object_accounts_object_id_put**](AccountsApi.md#object_accounts_object_id_put) | **PUT** /object/accounts/{objectId}/ | 
[**object_accounts_object_id_role_delete**](AccountsApi.md#object_accounts_object_id_role_delete) | **DELETE** /object/accounts/{objectId}/role | 
[**object_accounts_object_id_role_put**](AccountsApi.md#object_accounts_object_id_role_put) | **PUT** /object/accounts/{objectId}/role | 
[**object_accounts_object_id_role_report_get**](AccountsApi.md#object_accounts_object_id_role_report_get) | **GET** /object/accounts/{objectId}/role/report | 



## basic_accounts_admin_admin_user_post

> models::AccountPortalView basic_accounts_admin_admin_user_post(x_beam_gamertag, create_elevated_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_elevated_account_request** | Option<[**CreateElevatedAccountRequest**](CreateElevatedAccountRequest.md)> |  |  |

### Return type

[**models::AccountPortalView**](AccountPortalView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_admin_admin_users_get

> models::GetAdminsResponse basic_accounts_admin_admin_users_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetAdminsResponse**](GetAdminsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_admin_me_get

> models::AccountPortalView basic_accounts_admin_me_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AccountPortalView**](AccountPortalView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_available_device_id_get

> models::AccountAvailableResponse basic_accounts_available_device_id_get(device_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AccountAvailableResponse**](AccountAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_available_external_identity_get

> models::AccountAvailableResponse basic_accounts_available_external_identity_get(provider_service, user_id, x_beam_gamertag, provider_namespace)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_service** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**provider_namespace** | Option<**String**> |  |  |

### Return type

[**models::AccountAvailableResponse**](AccountAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_available_get

> models::AccountAvailableResponse basic_accounts_available_get(email, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AccountAvailableResponse**](AccountAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_available_third_party_get

> models::AccountAvailableResponse basic_accounts_available_third_party_get(third_party, token, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**third_party** | **String** |  | [required] |
**token** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AccountAvailableResponse**](AccountAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_email_update_confirm_post

> models::EmptyResponse basic_accounts_email_update_confirm_post(x_beam_gamertag, email_update_confirmation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**email_update_confirmation** | Option<[**EmailUpdateConfirmation**](EmailUpdateConfirmation.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_email_update_init_post

> models::EmptyResponse basic_accounts_email_update_init_post(x_beam_gamertag, email_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**email_update_request** | Option<[**EmailUpdateRequest**](EmailUpdateRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_external_identity_delete

> models::CommonResponse basic_accounts_external_identity_delete(x_beam_gamertag, delete_external_identity_api_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_external_identity_api_request** | Option<[**DeleteExternalIdentityApiRequest**](DeleteExternalIdentityApiRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_external_identity_post

> models::AttachExternalIdentityApiResponse basic_accounts_external_identity_post(x_beam_gamertag, attach_external_identity_api_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**attach_external_identity_api_request** | Option<[**AttachExternalIdentityApiRequest**](AttachExternalIdentityApiRequest.md)> |  |  |

### Return type

[**models::AttachExternalIdentityApiResponse**](AttachExternalIdentityApiResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_find_get

> models::Account basic_accounts_find_get(query, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_get_personally_identifiable_information_get

> models::AccountPersonallyIdentifiableInformationResponse basic_accounts_get_personally_identifiable_information_get(query, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AccountPersonallyIdentifiableInformationResponse**](AccountPersonallyIdentifiableInformationResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_me_device_delete

> models::AccountPlayerView basic_accounts_me_device_delete(x_beam_gamertag, delete_devices_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_devices_request** | Option<[**DeleteDevicesRequest**](DeleteDevicesRequest.md)> |  |  |

### Return type

[**models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_me_get

> models::AccountPlayerView basic_accounts_me_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_me_put

> models::AccountPlayerView basic_accounts_me_put(x_beam_gamertag, account_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**account_update** | Option<[**AccountUpdate**](AccountUpdate.md)> |  |  |

### Return type

[**models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_me_third_party_delete

> models::AccountPlayerView basic_accounts_me_third_party_delete(x_beam_gamertag, third_party_available_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**third_party_available_request** | Option<[**ThirdPartyAvailableRequest**](ThirdPartyAvailableRequest.md)> |  |  |

### Return type

[**models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_password_update_confirm_post

> models::EmptyResponse basic_accounts_password_update_confirm_post(x_beam_gamertag, password_update_confirmation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**password_update_confirmation** | Option<[**PasswordUpdateConfirmation**](PasswordUpdateConfirmation.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_password_update_init_post

> models::EmptyResponse basic_accounts_password_update_init_post(x_beam_gamertag, password_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**password_update_request** | Option<[**PasswordUpdateRequest**](PasswordUpdateRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_register_post

> models::AccountPlayerView basic_accounts_register_post(x_beam_gamertag, account_registration)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**account_registration** | Option<[**AccountRegistration**](AccountRegistration.md)> |  |  |

### Return type

[**models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_search_get

> models::AccountSearchResponse basic_accounts_search_get(query, page, pagesize, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**page** | **i32** |  | [required] |
**pagesize** | **i32** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AccountSearchResponse**](AccountSearchResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_signup_post

> models::CreateAccountWithCredsApiResponse basic_accounts_signup_post(x_beam_gamertag, create_account_with_creds_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_account_with_creds_request** | Option<[**CreateAccountWithCredsRequest**](CreateAccountWithCredsRequest.md)> |  |  |

### Return type

[**models::CreateAccountWithCredsApiResponse**](CreateAccountWithCredsApiResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_admin_email_put

> models::Account object_accounts_object_id_admin_email_put(object_id, x_beam_gamertag, email_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**email_update_request** | Option<[**EmailUpdateRequest**](EmailUpdateRequest.md)> |  |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_admin_forget_delete

> models::Account object_accounts_object_id_admin_forget_delete(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_admin_scope_delete

> models::EmptyResponse object_accounts_object_id_admin_scope_delete(object_id, x_beam_gamertag, delete_role)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_role** | Option<[**DeleteRole**](DeleteRole.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_admin_scope_put

> models::EmptyResponse object_accounts_object_id_admin_scope_put(object_id, x_beam_gamertag, update_role)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**update_role** | Option<[**UpdateRole**](UpdateRole.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_admin_third_party_delete

> models::EmptyResponse object_accounts_object_id_admin_third_party_delete(object_id, x_beam_gamertag, delete_third_party_association)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_third_party_association** | Option<[**DeleteThirdPartyAssociation**](DeleteThirdPartyAssociation.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_admin_third_party_put

> models::EmptyResponse object_accounts_object_id_admin_third_party_put(object_id, x_beam_gamertag, transfer_third_party_association)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**transfer_third_party_association** | Option<[**TransferThirdPartyAssociation**](TransferThirdPartyAssociation.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_available_roles_get

> models::AvailableRolesResponse object_accounts_object_id_available_roles_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AvailableRolesResponse**](AvailableRolesResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_put

> models::Account object_accounts_object_id_put(object_id, x_beam_gamertag, account_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**account_update** | Option<[**AccountUpdate**](AccountUpdate.md)> |  |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_role_delete

> models::EmptyResponse object_accounts_object_id_role_delete(object_id, x_beam_gamertag, delete_role)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_role** | Option<[**DeleteRole**](DeleteRole.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_role_put

> models::EmptyResponse object_accounts_object_id_role_put(object_id, x_beam_gamertag, update_role)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**update_role** | Option<[**UpdateRole**](UpdateRole.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_accounts_object_id_role_report_get

> models::AccountRolesReport object_accounts_object_id_role_report_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | AccountId of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::AccountRolesReport**](AccountRolesReport.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

