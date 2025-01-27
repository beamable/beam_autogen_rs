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
[**api_parties_post**](PartyApi.md#api_parties_post) | **POST** /api/parties | 



## api_parties_id_get

> models::Party api_parties_id_get(id, x_beam_scope, x_beam_gamertag)


Return the status of a party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_invite_delete

> serde_json::Value api_parties_id_invite_delete(id, x_beam_scope, x_beam_gamertag, cancel_invite_to_party)


Cancel party invitation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**cancel_invite_to_party** | Option<[**CancelInviteToParty**](CancelInviteToParty.md)> | Player to be uninvited |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_invite_post

> serde_json::Value api_parties_id_invite_post(id, x_beam_scope, x_beam_gamertag, invite_to_party)


Invite a player to a party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**invite_to_party** | Option<[**InviteToParty**](InviteToParty.md)> | Player to invite to the party |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_members_delete

> serde_json::Value api_parties_id_members_delete(id, x_beam_scope, x_beam_gamertag, leave_party)


Remove the requested player from the party. The leader is able to remove anyone. Others may  only remove themselves without error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**leave_party** | Option<[**LeaveParty**](LeaveParty.md)> | The leave party request |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_metadata_put

> models::Party api_parties_id_metadata_put(id, x_beam_scope, x_beam_gamertag, update_party)


Updates party state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**update_party** | Option<[**UpdateParty**](UpdateParty.md)> | Argument to pass to the party actor to update state. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_promote_put

> models::Party api_parties_id_promote_put(id, x_beam_scope, x_beam_gamertag, promote_new_leader)


Promote a party member to leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**promote_new_leader** | Option<[**PromoteNewLeader**](PromoteNewLeader.md)> | Player to promote to leader |  |

### Return type

[**models::Party**](Party.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_put

> models::Party api_parties_id_put(id, x_beam_scope, x_beam_gamertag)


Join a party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_post

> models::Party api_parties_post(x_beam_scope, x_beam_gamertag, create_party)


Create a party for the current player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_scope** | Option<**String**> | Customer and project scope. This should be in the form of '{customerId}.{projectId}'. This is only necessary when not using a JWT bearer token |  |
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**create_party** | Option<[**CreateParty**](CreateParty.md)> | Argument to pass to the party actor to initialize state. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

