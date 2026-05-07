# \SocialApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_social_blocked_delete**](SocialApi.md#basic_social_blocked_delete) | **DELETE** /basic/social/blocked | 
[**basic_social_blocked_post**](SocialApi.md#basic_social_blocked_post) | **POST** /basic/social/blocked | 
[**basic_social_friends_delete**](SocialApi.md#basic_social_friends_delete) | **DELETE** /basic/social/friends | 
[**basic_social_friends_import_post**](SocialApi.md#basic_social_friends_import_post) | **POST** /basic/social/friends/import | 
[**basic_social_friends_invite_delete**](SocialApi.md#basic_social_friends_invite_delete) | **DELETE** /basic/social/friends/invite | 
[**basic_social_friends_invite_post**](SocialApi.md#basic_social_friends_invite_post) | **POST** /basic/social/friends/invite | 
[**basic_social_friends_make_post**](SocialApi.md#basic_social_friends_make_post) | **POST** /basic/social/friends/make | 
[**basic_social_get**](SocialApi.md#basic_social_get) | **GET** /basic/social/ | 
[**basic_social_my_get**](SocialApi.md#basic_social_my_get) | **GET** /basic/social/my | 



## basic_social_blocked_delete

> models::FriendshipStatus basic_social_blocked_delete(x_beam_gamertag, player_id_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**player_id_request** | Option<[**PlayerIdRequest**](PlayerIdRequest.md)> |  |  |

### Return type

[**models::FriendshipStatus**](FriendshipStatus.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_social_blocked_post

> models::FriendshipStatus basic_social_blocked_post(x_beam_gamertag, player_id_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**player_id_request** | Option<[**PlayerIdRequest**](PlayerIdRequest.md)> |  |  |

### Return type

[**models::FriendshipStatus**](FriendshipStatus.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_social_friends_delete

> models::EmptyResponse basic_social_friends_delete(x_beam_gamertag, player_id_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**player_id_request** | Option<[**PlayerIdRequest**](PlayerIdRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_social_friends_import_post

> models::EmptyResponse basic_social_friends_import_post(x_beam_gamertag, import_friends_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**import_friends_request** | Option<[**ImportFriendsRequest**](ImportFriendsRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_social_friends_invite_delete

> models::EmptyResponse basic_social_friends_invite_delete(x_beam_gamertag, send_friend_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**send_friend_request** | Option<[**SendFriendRequest**](SendFriendRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_social_friends_invite_post

> models::EmptyResponse basic_social_friends_invite_post(x_beam_gamertag, send_friend_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**send_friend_request** | Option<[**SendFriendRequest**](SendFriendRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_social_friends_make_post

> models::CommonResponse basic_social_friends_make_post(x_beam_gamertag, make_friendship_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**make_friendship_request** | Option<[**MakeFriendshipRequest**](MakeFriendshipRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_social_get

> models::GetSocialStatusesResponse basic_social_get(player_ids, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_ids** | [**Vec<String>**](String.md) |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetSocialStatusesResponse**](GetSocialStatusesResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_social_my_get

> models::Social basic_social_my_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::Social**](Social.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

