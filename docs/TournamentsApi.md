# \TournamentsApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_tournaments_admin_player_get**](TournamentsApi.md#basic_tournaments_admin_player_get) | **GET** /basic/tournaments/admin/player | 
[**basic_tournaments_admin_player_put**](TournamentsApi.md#basic_tournaments_admin_player_put) | **PUT** /basic/tournaments/admin/player | 
[**basic_tournaments_champions_get**](TournamentsApi.md#basic_tournaments_champions_get) | **GET** /basic/tournaments/champions | 
[**basic_tournaments_get**](TournamentsApi.md#basic_tournaments_get) | **GET** /basic/tournaments/ | 
[**basic_tournaments_global_get**](TournamentsApi.md#basic_tournaments_global_get) | **GET** /basic/tournaments/global | 
[**basic_tournaments_groups_get**](TournamentsApi.md#basic_tournaments_groups_get) | **GET** /basic/tournaments/groups | 
[**basic_tournaments_me_get**](TournamentsApi.md#basic_tournaments_me_get) | **GET** /basic/tournaments/me | 
[**basic_tournaments_me_group_get**](TournamentsApi.md#basic_tournaments_me_group_get) | **GET** /basic/tournaments/me/group | 
[**basic_tournaments_post**](TournamentsApi.md#basic_tournaments_post) | **POST** /basic/tournaments/ | 
[**basic_tournaments_rewards_get**](TournamentsApi.md#basic_tournaments_rewards_get) | **GET** /basic/tournaments/rewards | 
[**basic_tournaments_rewards_post**](TournamentsApi.md#basic_tournaments_rewards_post) | **POST** /basic/tournaments/rewards | 
[**basic_tournaments_score_post**](TournamentsApi.md#basic_tournaments_score_post) | **POST** /basic/tournaments/score | 
[**basic_tournaments_search_groups_post**](TournamentsApi.md#basic_tournaments_search_groups_post) | **POST** /basic/tournaments/search/groups | 
[**basic_tournaments_standings_get**](TournamentsApi.md#basic_tournaments_standings_get) | **GET** /basic/tournaments/standings | 
[**basic_tournaments_standings_group_get**](TournamentsApi.md#basic_tournaments_standings_group_get) | **GET** /basic/tournaments/standings/group | 



## basic_tournaments_admin_player_get

> models::AdminGetPlayerStatusResponse basic_tournaments_admin_player_get(player_id, x_beam_gamertag, tournament_id, content_id, has_unclaimed_rewards)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**tournament_id** | Option<**String**> |  |  |
**content_id** | Option<**String**> |  |  |
**has_unclaimed_rewards** | Option<**bool**> |  |  |

### Return type

[**models::AdminGetPlayerStatusResponse**](AdminGetPlayerStatusResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_admin_player_put

> models::AdminPlayerStatus basic_tournaments_admin_player_put(x_beam_gamertag, update_player_status_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**update_player_status_request** | Option<[**UpdatePlayerStatusRequest**](UpdatePlayerStatusRequest.md)> |  |  |

### Return type

[**models::AdminPlayerStatus**](AdminPlayerStatus.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_champions_get

> models::GetChampionsResponse basic_tournaments_champions_get(tournament_id, cycles, x_beam_gamertag, content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tournament_id** | **String** |  | [required] |
**cycles** | **i32** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**content_id** | Option<**String**> |  |  |

### Return type

[**models::GetChampionsResponse**](GetChampionsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_get

> models::TournamentQueryResponse basic_tournaments_get(x_beam_gamertag, is_running, content_id, cycle)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**is_running** | Option<**bool**> |  |  |
**content_id** | Option<**String**> |  |  |
**cycle** | Option<**i32**> |  |  |

### Return type

[**models::TournamentQueryResponse**](TournamentQueryResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_global_get

> models::GetStandingsResponse basic_tournaments_global_get(tournament_id, x_beam_gamertag, max, focus, cycle, from, content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tournament_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**max** | Option<**i32**> |  |  |
**focus** | Option<**i64**> |  |  |
**cycle** | Option<**i32**> |  |  |
**from** | Option<**i32**> |  |  |
**content_id** | Option<**String**> |  |  |

### Return type

[**models::GetStandingsResponse**](GetStandingsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_groups_get

> models::GetGroupsResponse basic_tournaments_groups_get(tournament_id, x_beam_gamertag, max, focus, cycle, from, content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tournament_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**max** | Option<**i32**> |  |  |
**focus** | Option<**i64**> |  |  |
**cycle** | Option<**i32**> |  |  |
**from** | Option<**i32**> |  |  |
**content_id** | Option<**String**> |  |  |

### Return type

[**models::GetGroupsResponse**](GetGroupsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_me_get

> models::GetPlayerStatusResponse basic_tournaments_me_get(x_beam_gamertag, tournament_id, content_id, has_unclaimed_rewards)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**tournament_id** | Option<**String**> |  |  |
**content_id** | Option<**String**> |  |  |
**has_unclaimed_rewards** | Option<**bool**> |  |  |

### Return type

[**models::GetPlayerStatusResponse**](GetPlayerStatusResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_me_group_get

> models::GetGroupStatusResponse basic_tournaments_me_group_get(x_beam_gamertag, content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**content_id** | Option<**String**> |  |  |

### Return type

[**models::GetGroupStatusResponse**](GetGroupStatusResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_post

> models::PlayerStatus basic_tournaments_post(x_beam_gamertag, join_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**join_request** | Option<[**JoinRequest**](JoinRequest.md)> |  |  |

### Return type

[**models::PlayerStatus**](PlayerStatus.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_rewards_get

> models::RewardsResponse basic_tournaments_rewards_get(x_beam_gamertag, tournament_id, content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**tournament_id** | Option<**String**> |  |  |
**content_id** | Option<**String**> |  |  |

### Return type

[**models::RewardsResponse**](RewardsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_rewards_post

> models::RewardsResponse basic_tournaments_rewards_post(x_beam_gamertag, rewards_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**rewards_request** | Option<[**RewardsRequest**](RewardsRequest.md)> |  |  |

### Return type

[**models::RewardsResponse**](RewardsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_score_post

> models::EmptyResponse basic_tournaments_score_post(x_beam_gamertag, score_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**score_request** | Option<[**ScoreRequest**](ScoreRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_search_groups_post

> models::GetStatusForGroupsResponse basic_tournaments_search_groups_post(x_beam_gamertag, get_status_for_groups_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**get_status_for_groups_request** | Option<[**GetStatusForGroupsRequest**](GetStatusForGroupsRequest.md)> |  |  |

### Return type

[**models::GetStatusForGroupsResponse**](GetStatusForGroupsResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_standings_get

> models::GetStandingsResponse basic_tournaments_standings_get(tournament_id, x_beam_gamertag, max, focus, cycle, from, content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tournament_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**max** | Option<**i32**> |  |  |
**focus** | Option<**i64**> |  |  |
**cycle** | Option<**i32**> |  |  |
**from** | Option<**i32**> |  |  |
**content_id** | Option<**String**> |  |  |

### Return type

[**models::GetStandingsResponse**](GetStandingsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_tournaments_standings_group_get

> models::GetStandingsResponse basic_tournaments_standings_group_get(tournament_id, x_beam_gamertag, max, focus, cycle, from, content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tournament_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**max** | Option<**i32**> |  |  |
**focus** | Option<**i64**> |  |  |
**cycle** | Option<**i32**> |  |  |
**from** | Option<**i32**> |  |  |
**content_id** | Option<**String**> |  |  |

### Return type

[**models::GetStandingsResponse**](GetStandingsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

