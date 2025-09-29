# CreateServicePlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**mongo_hosts** | Option<**String**> |  | [optional]
**mongo_sharded** | Option<**bool**> |  | [optional]
**mongo_tls** | Option<**bool**> |  | [optional]
**redis_shards** | Option<[**Vec<models::ServicePlanActorRedisShardRequest>**](ServicePlanActorRedisShardRequest.md)> |  | [optional]
**message_bus_analytics** | Option<**Vec<String>**> |  | [optional]
**message_bus_common** | Option<**Vec<String>**> |  | [optional]
**mongo_srv_address** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


