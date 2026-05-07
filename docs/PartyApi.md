# \PartyApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_parties_id_get**](PartyApi.md#api_parties_id_get) | **GET** /api/parties/{id} | 
[**api_parties_id_invite_delete**](PartyApi.md#api_parties_id_invite_delete) | **DELETE** /api/parties/{id}/invite | 
[**api_parties_id_invite_post**](PartyApi.md#api_parties_id_invite_post) | **POST** /api/parties/{id}/invite | 
[**api_parties_id_members_delete**](PartyApi.md#api_parties_id_members_delete) | **DELETE** /api/parties/{id}/members | 
[**api_parties_id_metadata_put**](PartyApi.md#api_parties_id_metadata_put) | **PUT** /api/parties/{id}/metadata | 
[**api_parties_id_promote_put**](PartyApi.md#api_parties_id_promote_put) | **PUT** /api/parties/{id}/promote | 
[**api_parties_id_put**](PartyApi.md#api_parties_id_put) | **PUT** /api/parties/{id} | 
[**api_parties_id_tags_put**](PartyApi.md#api_parties_id_tags_put) | **PUT** /api/parties/{id}/tags | 
[**api_parties_post**](PartyApi.md#api_parties_post) | **POST** /api/parties | 
[**api_parties_put**](PartyApi.md#api_parties_put) | **PUT** /api/parties | 



## api_parties_id_get

> models::Party api_parties_id_get(id, x_beam_gamertag, x_beam_timeout)


Return the status of a party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_invite_delete

> serde_json::Value api_parties_id_invite_delete(id, x_beam_gamertag, x_beam_timeout, cancel_invite_to_party)


Cancel party invitation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**cancel_invite_to_party** | Option<[**CancelInviteToParty**](CancelInviteToParty.md)> | Player to be uninvited |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_invite_post

> serde_json::Value api_parties_id_invite_post(id, x_beam_gamertag, x_beam_timeout, invite_to_party)


Invite a player to a party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**invite_to_party** | Option<[**InviteToParty**](InviteToParty.md)> | Player to invite to the party |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_members_delete

> serde_json::Value api_parties_id_members_delete(id, x_beam_gamertag, x_beam_timeout, leave_party)


Remove the requested player from the party. The leader is able to remove anyone. Others may only remove themselves without error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**leave_party** | Option<[**LeaveParty**](LeaveParty.md)> | The leave party request |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_metadata_put

> models::Party api_parties_id_metadata_put(id, x_beam_gamertag, x_beam_timeout, update_party)


Updates party state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**update_party** | Option<[**UpdateParty**](UpdateParty.md)> | Argument to pass to the party actor to update state. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_promote_put

> models::Party api_parties_id_promote_put(id, x_beam_gamertag, x_beam_timeout, promote_new_leader)


Promote a party member to leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**promote_new_leader** | Option<[**PromoteNewLeader**](PromoteNewLeader.md)> | Player to promote to leader |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_put

> models::Party api_parties_id_put(id, x_beam_gamertag, x_beam_timeout, party_member_tags)


Join a party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**party_member_tags** | Option<[**PartyMemberTags**](PartyMemberTags.md)> | The list of player tags when joining the party |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_tags_put

> models::Party api_parties_id_tags_put(id, x_beam_gamertag, x_beam_timeout, update_party_tags)


Update the tags for the player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**update_party_tags** | Option<[**UpdatePartyTags**](UpdatePartyTags.md)> | Tags to update for the player. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_post

> models::Party api_parties_post(x_beam_gamertag, x_beam_timeout, create_party)


Create a party for the current player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**create_party** | Option<[**CreateParty**](CreateParty.md)> | Argument to pass to the party actor to initialize state. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_put

> models::Party api_parties_put(x_beam_gamertag, x_beam_timeout, party)


Exposes the internal \"SetParty\" behavior as an Admin only endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |
**party** | Option<[**Party**](Party.md)> | The party to create or replace. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

