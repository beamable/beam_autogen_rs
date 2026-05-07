# \LeaderboardsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_leaderboards_assignment_get**](LeaderboardsApi.md#basic_leaderboards_assignment_get) | **GET** /basic/leaderboards/assignment | 
[**basic_leaderboards_list_get**](LeaderboardsApi.md#basic_leaderboards_list_get) | **GET** /basic/leaderboards/list | 
[**basic_leaderboards_player_get**](LeaderboardsApi.md#basic_leaderboards_player_get) | **GET** /basic/leaderboards/player | 
[**basic_leaderboards_uid_get**](LeaderboardsApi.md#basic_leaderboards_uid_get) | **GET** /basic/leaderboards/uid | 
[**object_leaderboards_object_id_assignment_delete**](LeaderboardsApi.md#object_leaderboards_object_id_assignment_delete) | **DELETE** /object/leaderboards/{objectId}/assignment | 
[**object_leaderboards_object_id_assignment_get**](LeaderboardsApi.md#object_leaderboards_object_id_assignment_get) | **GET** /object/leaderboards/{objectId}/assignment | 
[**object_leaderboards_object_id_delete**](LeaderboardsApi.md#object_leaderboards_object_id_delete) | **DELETE** /object/leaderboards/{objectId}/ | 
[**object_leaderboards_object_id_details_get**](LeaderboardsApi.md#object_leaderboards_object_id_details_get) | **GET** /object/leaderboards/{objectId}/details | 
[**object_leaderboards_object_id_entries_delete**](LeaderboardsApi.md#object_leaderboards_object_id_entries_delete) | **DELETE** /object/leaderboards/{objectId}/entries | 
[**object_leaderboards_object_id_entry_delete**](LeaderboardsApi.md#object_leaderboards_object_id_entry_delete) | **DELETE** /object/leaderboards/{objectId}/entry | 
[**object_leaderboards_object_id_entry_put**](LeaderboardsApi.md#object_leaderboards_object_id_entry_put) | **PUT** /object/leaderboards/{objectId}/entry | 
[**object_leaderboards_object_id_freeze_delete**](LeaderboardsApi.md#object_leaderboards_object_id_freeze_delete) | **DELETE** /object/leaderboards/{objectId}/freeze | 
[**object_leaderboards_object_id_freeze_put**](LeaderboardsApi.md#object_leaderboards_object_id_freeze_put) | **PUT** /object/leaderboards/{objectId}/freeze | 
[**object_leaderboards_object_id_friends_get**](LeaderboardsApi.md#object_leaderboards_object_id_friends_get) | **GET** /object/leaderboards/{objectId}/friends | 
[**object_leaderboards_object_id_matches_get**](LeaderboardsApi.md#object_leaderboards_object_id_matches_get) | **GET** /object/leaderboards/{objectId}/matches | 
[**object_leaderboards_object_id_membership_get**](LeaderboardsApi.md#object_leaderboards_object_id_membership_get) | **GET** /object/leaderboards/{objectId}/membership | 
[**object_leaderboards_object_id_partition_get**](LeaderboardsApi.md#object_leaderboards_object_id_partition_get) | **GET** /object/leaderboards/{objectId}/partition | 
[**object_leaderboards_object_id_post**](LeaderboardsApi.md#object_leaderboards_object_id_post) | **POST** /object/leaderboards/{objectId}/ | 
[**object_leaderboards_object_id_ranks_get**](LeaderboardsApi.md#object_leaderboards_object_id_ranks_get) | **GET** /object/leaderboards/{objectId}/ranks | 
[**object_leaderboards_object_id_swap_put**](LeaderboardsApi.md#object_leaderboards_object_id_swap_put) | **PUT** /object/leaderboards/{objectId}/swap | 
[**object_leaderboards_object_id_view_get**](LeaderboardsApi.md#object_leaderboards_object_id_view_get) | **GET** /object/leaderboards/{objectId}/view | 



## basic_leaderboards_assignment_get

> models::LeaderboardAssignmentInfo basic_leaderboards_assignment_get(board_id, x_beam_gamertag, join_board)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**join_board** | Option<**bool**> |  |  |

### Return type

[**models::LeaderboardAssignmentInfo**](LeaderboardAssignmentInfo.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_leaderboards_list_get

> models::LeaderboardListResponse basic_leaderboards_list_get(x_beam_gamertag, skip, limit, prefix, include_partitions)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**skip** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**prefix** | Option<**String**> |  |  |
**include_partitions** | Option<**bool**> |  |  |

### Return type

[**models::LeaderboardListResponse**](LeaderboardListResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_leaderboards_player_get

> models::ListLeaderBoardViewResponse basic_leaderboards_player_get(dbid, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbid** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ListLeaderBoardViewResponse**](ListLeaderBoardViewResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_leaderboards_uid_get

> models::LeaderboardUidResponse basic_leaderboards_uid_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::LeaderboardUidResponse**](LeaderboardUidResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_assignment_delete

> models::CommonResponse object_leaderboards_object_id_assignment_delete(object_id, x_beam_gamertag, leaderboard_remove_cache_entry_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**leaderboard_remove_cache_entry_request** | Option<[**LeaderboardRemoveCacheEntryRequest**](LeaderboardRemoveCacheEntryRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_assignment_get

> models::LeaderboardAssignmentInfo object_leaderboards_object_id_assignment_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::LeaderboardAssignmentInfo**](LeaderboardAssignmentInfo.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_delete

> models::CommonResponse object_leaderboards_object_id_delete(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_details_get

> models::LeaderboardDetails object_leaderboards_object_id_details_get(object_id, x_beam_gamertag, from, max)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**from** | Option<**i32**> |  |  |
**max** | Option<**i32**> |  |  |

### Return type

[**models::LeaderboardDetails**](LeaderboardDetails.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_entries_delete

> models::CommonResponse object_leaderboards_object_id_entries_delete(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_entry_delete

> models::CommonResponse object_leaderboards_object_id_entry_delete(object_id, x_beam_gamertag, leaderboard_remove_entry_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**leaderboard_remove_entry_request** | Option<[**LeaderboardRemoveEntryRequest**](LeaderboardRemoveEntryRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_entry_put

> models::CommonResponse object_leaderboards_object_id_entry_put(object_id, x_beam_gamertag, leaderboard_add_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**leaderboard_add_request** | Option<[**LeaderboardAddRequest**](LeaderboardAddRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_freeze_delete

> models::CommonResponse object_leaderboards_object_id_freeze_delete(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_freeze_put

> models::CommonResponse object_leaderboards_object_id_freeze_put(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_friends_get

> models::LeaderBoardViewResponse object_leaderboards_object_id_friends_get(object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::LeaderBoardViewResponse**](LeaderBoardViewResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_matches_get

> models::MatchMakingMatchesPvpResponse object_leaderboards_object_id_matches_get(pool_size, windows, window_size, object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_size** | **i32** |  | [required] |
**windows** | **i32** |  | [required] |
**window_size** | **i32** |  | [required] |
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::MatchMakingMatchesPvpResponse**](MatchMakingMatchesPvpResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_membership_get

> models::LeaderboardMembershipResponse object_leaderboards_object_id_membership_get(player_id, object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** |  | [required] |
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::LeaderboardMembershipResponse**](LeaderboardMembershipResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_partition_get

> models::LeaderboardPartitionInfo object_leaderboards_object_id_partition_get(player_id, object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** |  | [required] |
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::LeaderboardPartitionInfo**](LeaderboardPartitionInfo.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_post

> models::CommonResponse object_leaderboards_object_id_post(object_id, x_beam_gamertag, leaderboard_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**leaderboard_create_request** | Option<[**LeaderboardCreateRequest**](LeaderboardCreateRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_ranks_get

> models::LeaderBoardViewResponse object_leaderboards_object_id_ranks_get(ids, object_id, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** |  | [required] |
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::LeaderBoardViewResponse**](LeaderBoardViewResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_swap_put

> models::CommonResponse object_leaderboards_object_id_swap_put(object_id, x_beam_gamertag, leaderboard_swap_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**leaderboard_swap_request** | Option<[**LeaderboardSwapRequest**](LeaderboardSwapRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_leaderboards_object_id_view_get

> models::LeaderBoardViewResponse object_leaderboards_object_id_view_get(object_id, x_beam_gamertag, max, focus, friends, from, outlier, guild)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Leaderboard ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**max** | Option<**i32**> |  |  |
**focus** | Option<**i64**> |  |  |
**friends** | Option<**bool**> |  |  |
**from** | Option<**i32**> |  |  |
**outlier** | Option<**i64**> |  |  |
**guild** | Option<**bool**> |  |  |

### Return type

[**models::LeaderBoardViewResponse**](LeaderBoardViewResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

