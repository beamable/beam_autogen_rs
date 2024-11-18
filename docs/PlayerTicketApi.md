# \PlayerTicketApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_matchmaking_tickets_get**](PlayerTicketApi.md#api_players_player_id_matchmaking_tickets_get) | **GET** /api/players/{playerId}/matchmaking/tickets | 



## api_players_player_id_matchmaking_tickets_get

> models::TicketQueryResponse api_players_player_id_matchmaking_tickets_get(player_id)


Fetch the requested player's active Ticket information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player Id | [required] |

### Return type

[**models::TicketQueryResponse**](TicketQueryResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

