# JobDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**owner** | Option<**String**> |  | [optional]
**triggers** | Option<[**Vec<models::JobDefinitionSaveRequestTriggersInner>**](JobDefinitionSaveRequestTriggersInner.md)> |  | [optional]
**job_action** | Option<[**models::JobDefinitionSaveRequestJobAction**](JobDefinitionSaveRequestJobAction.md)> |  | [optional]
**retry_policy** | Option<[**models::JobRetryPolicy**](JobRetryPolicy.md)> |  | [optional]
**last_update** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**source** | Option<**String**> |  | [optional]
**nonce** | Option<**String**> |  | [optional]
**is_unique** | Option<**bool**> |  | [optional]
**suspended_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**analytics** | Option<[**models::JobAnalytics**](JobAnalytics.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


