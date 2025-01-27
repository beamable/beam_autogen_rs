# \TicketApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_matchmaking_tickets_get**](TicketApi.md#api_matchmaking_tickets_get) | **GET** /api/matchmaking/tickets | 
[**api_matchmaking_tickets_id_delete**](TicketApi.md#api_matchmaking_tickets_id_delete) | **DELETE** /api/matchmaking/tickets/{id} | 
[**api_matchmaking_tickets_id_get**](TicketApi.md#api_matchmaking_tickets_id_get) | **GET** /api/matchmaking/tickets/{id} | 
[**api_matchmaking_tickets_post**](TicketApi.md#api_matchmaking_tickets_post) | **POST** /api/matchmaking/tickets | 



## api_matchmaking_tickets_get

> models::TicketQueryResponse api_matchmaking_tickets_get(players, include_inactive, skip, limit, x_beam_scope, x_beam_gamertag)


Query for active tickets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**players** | Option<[**Vec<String>**](String.md)> |  |  |
**include_inactive** | Option<**bool**> |  |  |
**skip** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::TicketQueryResponse**](TicketQueryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_matchmaking_tickets_id_delete

> serde_json::Value api_matchmaking_tickets_id_delete(id, x_beam_scope, x_beam_gamertag)


Cancel a pending ticket. If no ticket with the id exists, this will  still return a 204.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_matchmaking_tickets_id_get

> models::Ticket api_matchmaking_tickets_id_get(id, x_beam_scope, x_beam_gamertag)


Fetch a ticket by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Ticket ID | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::Ticket**](Ticket.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_matchmaking_tickets_post

> models::TicketReservationResponse api_matchmaking_tickets_post(x_beam_scope, x_beam_gamertag, ticket_reservation_request)


Create a ticket representing 1 or more players to be matched  with others.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**ticket_reservation_request** | Option<[**TicketReservationRequest**](TicketReservationRequest.md)> |  |  |

### Return type

[**models::TicketReservationResponse**](TicketReservationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

