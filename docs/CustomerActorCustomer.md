# CustomerActorCustomer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | **i64** |  | 
**name** | **String** |  | 
**alias** | Option<**String**> |  | [optional]
**realms** | Option<[**Vec<models::Realm>**](Realm.md)> |  | [optional]
**accounts** | Option<[**Vec<models::CustomerActorAccount>**](CustomerActorAccount.md)> |  | [optional][readonly]
**payment_status** | Option<[**models::PaymentStatus**](PaymentStatus.md)> |  | [optional]
**activation_status** | Option<[**models::ActivationStatus**](ActivationStatus.md)> |  | [optional]
**contact** | Option<**String**> |  | [optional]
**stripe_customer_id** | Option<**String**> |  | [optional]
**requires_custom_tier** | Option<**bool**> |  | [optional]
**created** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**config** | Option<**std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


