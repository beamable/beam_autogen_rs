# \TicketApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_matchmaking_tickets_get**](TicketApi.md#api_matchmaking_tickets_get) | **GET** /api/matchmaking/tickets | 
[**api_matchmaking_tickets_id_delete**](TicketApi.md#api_matchmaking_tickets_id_delete) | **DELETE** /api/matchmaking/tickets/{id} | 
[**api_matchmaking_tickets_id_get**](TicketApi.md#api_matchmaking_tickets_id_get) | **GET** /api/matchmaking/tickets/{id} | 
[**api_matchmaking_tickets_post**](TicketApi.md#api_matchmaking_tickets_post) | **POST** /api/matchmaking/tickets | 



## api_matchmaking_tickets_get

> models::TicketQueryResponse api_matchmaking_tickets_get(x_beam_gamertag, x_beam_timeout, players, include_inactive, skip, limit)


Query for active tickets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**players** | Option<[**Vec<String>**](String.md)> |  |  |
**include_inactive** | Option<**bool**> |  |  |
**skip** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::TicketQueryResponse**](TicketQueryResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_matchmaking_tickets_id_delete

> serde_json::Value api_matchmaking_tickets_id_delete(id, x_beam_gamertag, x_beam_timeout)


Cancel a pending ticket. If no ticket with the id exists, this will still return a 204.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Ticket ID to cancel. | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_matchmaking_tickets_id_get

> models::Ticket api_matchmaking_tickets_id_get(id, x_beam_gamertag, x_beam_timeout)


Fetch a ticket by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Ticket ID | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::Ticket**](Ticket.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_matchmaking_tickets_post

> models::TicketReservationResponse api_matchmaking_tickets_post(x_beam_gamertag, x_beam_timeout, ticket_reservation_request)


Create a ticket representing 1 or more players to be matched with others.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**ticket_reservation_request** | Option<[**TicketReservationRequest**](TicketReservationRequest.md)> | Ticket reservation request specifying players and match types. |  |

### Return type

[**models::TicketReservationResponse**](TicketReservationResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

