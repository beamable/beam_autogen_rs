# CustomerActorAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**i64**> |  | [optional]
**created_time_ms** | Option<**i64**> |  | [optional]
**updated_time_ms** | Option<**i64**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**password_raw** | Option<**String**> |  | [optional]
**password** | Option<**String**> |  | [optional]
**realm_associations** | Option<[**Vec<models::RealmAssociation>**](RealmAssociation.md)> |  | [optional]
**third_party_associations** | Option<[**Vec<models::CustomerActorThirdPartyAssociation>**](CustomerActorThirdPartyAssociation.md)> |  | [optional]
**external** | Option<[**HashSet<models::CustomerActorExternalIdentity>**](CustomerActorExternalIdentity.md)> |  | [optional]
**username** | Option<**String**> |  | [optional]
**country** | Option<**String**> |  | [optional]
**language** | Option<**String**> |  | [optional]
**role_string** | Option<**String**> |  | [optional]
**roles** | Option<[**Vec<models::RoleAssociation>**](RoleAssociation.md)> |  | [optional]
**device_ids** | Option<**Vec<String>**> |  | [optional]
**realm_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


