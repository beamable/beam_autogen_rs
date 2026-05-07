# Party

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**restriction** | Option<**String**> |  | [optional]
**leader** | Option<**String**> |  | [optional]
**members** | Option<**Vec<String>**> |  | [optional][readonly]
**max_size** | Option<**i32**> |  | [optional]
**pending_invites** | Option<**Vec<String>**> |  | [optional][readonly]
**created** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**members_tags** | Option<[**std::collections::HashMap<String, models::TagList>**](TagList.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


