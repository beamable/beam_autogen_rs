# \GroupsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**object_groups_object_id_apply_post**](GroupsApi.md#object_groups_object_id_apply_post) | **POST** /object/groups/{objectId}/apply | 
[**object_groups_object_id_delete**](GroupsApi.md#object_groups_object_id_delete) | **DELETE** /object/groups/{objectId}/ | 
[**object_groups_object_id_donations_claim_put**](GroupsApi.md#object_groups_object_id_donations_claim_put) | **PUT** /object/groups/{objectId}/donations/claim | 
[**object_groups_object_id_donations_post**](GroupsApi.md#object_groups_object_id_donations_post) | **POST** /object/groups/{objectId}/donations | 
[**object_groups_object_id_donations_put**](GroupsApi.md#object_groups_object_id_donations_put) | **PUT** /object/groups/{objectId}/donations | 
[**object_groups_object_id_get**](GroupsApi.md#object_groups_object_id_get) | **GET** /object/groups/{objectId}/ | 
[**object_groups_object_id_invite_post**](GroupsApi.md#object_groups_object_id_invite_post) | **POST** /object/groups/{objectId}/invite | 
[**object_groups_object_id_kick_post**](GroupsApi.md#object_groups_object_id_kick_post) | **POST** /object/groups/{objectId}/kick | 
[**object_groups_object_id_member_delete**](GroupsApi.md#object_groups_object_id_member_delete) | **DELETE** /object/groups/{objectId}/member | 
[**object_groups_object_id_petition_post**](GroupsApi.md#object_groups_object_id_petition_post) | **POST** /object/groups/{objectId}/petition | 
[**object_groups_object_id_put**](GroupsApi.md#object_groups_object_id_put) | **PUT** /object/groups/{objectId}/ | 
[**object_groups_object_id_role_put**](GroupsApi.md#object_groups_object_id_role_put) | **PUT** /object/groups/{objectId}/role | 



## object_groups_object_id_apply_post

> models::CommonResponse object_groups_object_id_apply_post(object_id, x_beam_gamertag, group_application)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**group_application** | Option<[**GroupApplication**](GroupApplication.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_delete

> models::CommonResponse object_groups_object_id_delete(object_id, x_beam_gamertag, disband_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**disband_request** | Option<[**DisbandRequest**](DisbandRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_donations_claim_put

> models::EmptyResponse object_groups_object_id_donations_claim_put(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_donations_post

> models::EmptyResponse object_groups_object_id_donations_post(object_id, x_beam_gamertag, create_donation_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**create_donation_request** | Option<[**CreateDonationRequest**](CreateDonationRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_donations_put

> models::EmptyResponse object_groups_object_id_donations_put(object_id, x_beam_gamertag, make_donation_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**make_donation_request** | Option<[**MakeDonationRequest**](MakeDonationRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_get

> models::Group object_groups_object_id_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::Group**](Group.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_invite_post

> models::CommonResponse object_groups_object_id_invite_post(object_id, x_beam_gamertag, group_invite)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**group_invite** | Option<[**GroupInvite**](GroupInvite.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_kick_post

> models::GroupMembershipResponse object_groups_object_id_kick_post(object_id, x_beam_gamertag, kick_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**kick_request** | Option<[**KickRequest**](KickRequest.md)> |  |  |

### Return type

[**models::GroupMembershipResponse**](GroupMembershipResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_member_delete

> models::GroupMembershipResponse object_groups_object_id_member_delete(object_id, x_beam_gamertag, kick_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**kick_request** | Option<[**KickRequest**](KickRequest.md)> |  |  |

### Return type

[**models::GroupMembershipResponse**](GroupMembershipResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_petition_post

> models::CommonResponse object_groups_object_id_petition_post(object_id, x_beam_gamertag, group_application)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**group_application** | Option<[**GroupApplication**](GroupApplication.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_put

> models::CommonResponse object_groups_object_id_put(object_id, x_beam_gamertag, group_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**group_update** | Option<[**GroupUpdate**](GroupUpdate.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_groups_object_id_role_put

> models::CommonResponse object_groups_object_id_role_put(object_id, x_beam_gamertag, role_change_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**role_change_request** | Option<[**RoleChangeRequest**](RoleChangeRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

