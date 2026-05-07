# CreateLobby

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**has_name** | Option<**bool**> |  | [optional][readonly]
**description** | Option<**String**> |  | [optional]
**has_description** | Option<**bool**> |  | [optional][readonly]
**restriction** | Option<[**models::LobbyRestriction**](LobbyRestriction.md)> |  | [optional]
**has_restriction** | Option<**bool**> |  | [optional][readonly]
**match_type** | Option<**String**> |  | [optional]
**has_match_type** | Option<**bool**> |  | [optional][readonly]
**player_tags** | Option<[**Vec<models::Tag>**](Tag.md)> |  | [optional][readonly]
**passcode_length** | Option<**i32**> |  | [optional]
**has_passcode_length** | Option<**bool**> |  | [optional][readonly]
**max_players** | Option<**i32**> |  | [optional]
**has_max_players** | Option<**bool**> |  | [optional][readonly]
**data** | Option<**std::collections::HashMap<String, String>**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


