# \GroupUsersApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**object_group_users_object_id_availability_get**](GroupUsersApi.md#object_group_users_object_id_availability_get) | **GET** /object/group-users/{objectId}/availability | 
[**object_group_users_object_id_get**](GroupUsersApi.md#object_group_users_object_id_get) | **GET** /object/group-users/{objectId}/ | 
[**object_group_users_object_id_group_post**](GroupUsersApi.md#object_group_users_object_id_group_post) | **POST** /object/group-users/{objectId}/group | 
[**object_group_users_object_id_join_delete**](GroupUsersApi.md#object_group_users_object_id_join_delete) | **DELETE** /object/group-users/{objectId}/join | 
[**object_group_users_object_id_join_post**](GroupUsersApi.md#object_group_users_object_id_join_post) | **POST** /object/group-users/{objectId}/join | 
[**object_group_users_object_id_recommended_get**](GroupUsersApi.md#object_group_users_object_id_recommended_get) | **GET** /object/group-users/{objectId}/recommended | 
[**object_group_users_object_id_search_get**](GroupUsersApi.md#object_group_users_object_id_search_get) | **GET** /object/group-users/{objectId}/search | 



## object_group_users_object_id_availability_get

> models::AvailabilityResponse object_group_users_object_id_availability_get(r#type, object_id, x_beam_gamertag, name, tag, sub_group)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | [**GroupType**](GroupType.md) |  | [required] |
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**name** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**sub_group** | Option<**bool**> |  |  |

### Return type

[**models::AvailabilityResponse**](AvailabilityResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_group_users_object_id_get

> models::GroupUser object_group_users_object_id_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GroupUser**](GroupUser.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_group_users_object_id_group_post

> models::GroupCreateResponse object_group_users_object_id_group_post(object_id, x_beam_gamertag, group_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**group_create** | Option<[**GroupCreate**](GroupCreate.md)> |  |  |

### Return type

[**models::GroupCreateResponse**](GroupCreateResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_group_users_object_id_join_delete

> models::GroupMembershipResponse object_group_users_object_id_join_delete(object_id, x_beam_gamertag, group_membership_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**group_membership_request** | Option<[**GroupMembershipRequest**](GroupMembershipRequest.md)> |  |  |

### Return type

[**models::GroupMembershipResponse**](GroupMembershipResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_group_users_object_id_join_post

> models::GroupMembershipResponse object_group_users_object_id_join_post(object_id, x_beam_gamertag, group_membership_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**group_membership_request** | Option<[**GroupMembershipRequest**](GroupMembershipRequest.md)> |  |  |

### Return type

[**models::GroupMembershipResponse**](GroupMembershipResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_group_users_object_id_recommended_get

> models::GroupSearchResponse object_group_users_object_id_recommended_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GroupSearchResponse**](GroupSearchResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_group_users_object_id_search_get

> models::GroupSearchResponse object_group_users_object_id_search_get(r#type, object_id, x_beam_gamertag, name, score_min, sort_field, user_score, has_slots, enrollment_types, offset, score_max, sub_group, sort_value, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | [**GroupType**](GroupType.md) |  | [required] |
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**name** | Option<**String**> |  |  |
**score_min** | Option<**i64**> |  |  |
**sort_field** | Option<**String**> |  |  |
**user_score** | Option<**i64**> |  |  |
**has_slots** | Option<**bool**> |  |  |
**enrollment_types** | Option<**String**> |  |  |
**offset** | Option<**i32**> |  |  |
**score_max** | Option<**i64**> |  |  |
**sub_group** | Option<**bool**> |  |  |
**sort_value** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::GroupSearchResponse**](GroupSearchResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

