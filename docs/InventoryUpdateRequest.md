# InventoryUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currencies** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**currency_properties** | Option<[**std::collections::HashMap<String, Vec<models::CurrencyProperty>>**](Vec.md)> |  | [optional]
**apply_vip_bonus** | Option<**bool**> |  | [optional]
**update_items** | Option<[**Vec<models::ItemUpdateRequest>**](ItemUpdateRequest.md)> |  | [optional]
**new_items** | Option<[**Vec<models::ItemCreateRequest>**](ItemCreateRequest.md)> |  | [optional]
**transaction** | Option<**String**> |  | [optional]
**delete_items** | Option<[**Vec<models::ItemDeleteRequest>**](ItemDeleteRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


