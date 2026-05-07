# \ContentApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_content_binary_post**](ContentApi.md#basic_content_binary_post) | **POST** /basic/content/binary | 
[**basic_content_binary_private_urls_post**](ContentApi.md#basic_content_binary_private_urls_post) | **POST** /basic/content/binary/private/urls | 
[**basic_content_content_get**](ContentApi.md#basic_content_content_get) | **GET** /basic/content/content | 
[**basic_content_localizations_delete**](ContentApi.md#basic_content_localizations_delete) | **DELETE** /basic/content/localizations | 
[**basic_content_localizations_get**](ContentApi.md#basic_content_localizations_get) | **GET** /basic/content/localizations | 
[**basic_content_localizations_put**](ContentApi.md#basic_content_localizations_put) | **PUT** /basic/content/localizations | 
[**basic_content_manifest_checksum_get**](ContentApi.md#basic_content_manifest_checksum_get) | **GET** /basic/content/manifest/checksum | 
[**basic_content_manifest_checksums_get**](ContentApi.md#basic_content_manifest_checksums_get) | **GET** /basic/content/manifest/checksums | 
[**basic_content_manifest_diffs_get**](ContentApi.md#basic_content_manifest_diffs_get) | **GET** /basic/content/manifest/diffs | 
[**basic_content_manifest_exact_get**](ContentApi.md#basic_content_manifest_exact_get) | **GET** /basic/content/manifest/exact | 
[**basic_content_manifest_get**](ContentApi.md#basic_content_manifest_get) | **GET** /basic/content/manifest | 
[**basic_content_manifest_history_get**](ContentApi.md#basic_content_manifest_history_get) | **GET** /basic/content/manifest/history | 
[**basic_content_manifest_post**](ContentApi.md#basic_content_manifest_post) | **POST** /basic/content/manifest | 
[**basic_content_manifest_private_get**](ContentApi.md#basic_content_manifest_private_get) | **GET** /basic/content/manifest/private | 
[**basic_content_manifest_private_json_get**](ContentApi.md#basic_content_manifest_private_json_get) | **GET** /basic/content/manifest/private/json | 
[**basic_content_manifest_public_get**](ContentApi.md#basic_content_manifest_public_get) | **GET** /basic/content/manifest/public | 
[**basic_content_manifest_public_json_get**](ContentApi.md#basic_content_manifest_public_json_get) | **GET** /basic/content/manifest/public/json | 
[**basic_content_manifest_pull_post**](ContentApi.md#basic_content_manifest_pull_post) | **POST** /basic/content/manifest/pull | 
[**basic_content_manifest_repeat_put**](ContentApi.md#basic_content_manifest_repeat_put) | **PUT** /basic/content/manifest/repeat | 
[**basic_content_manifests_archive_post**](ContentApi.md#basic_content_manifests_archive_post) | **POST** /basic/content/manifests/archive | 
[**basic_content_manifests_get**](ContentApi.md#basic_content_manifests_get) | **GET** /basic/content/manifests | 
[**basic_content_manifests_pull_post**](ContentApi.md#basic_content_manifests_pull_post) | **POST** /basic/content/manifests/pull | 
[**basic_content_manifests_unarchive_post**](ContentApi.md#basic_content_manifests_unarchive_post) | **POST** /basic/content/manifests/unarchive | 
[**basic_content_post**](ContentApi.md#basic_content_post) | **POST** /basic/content/ | 
[**basic_content_text_post**](ContentApi.md#basic_content_text_post) | **POST** /basic/content/text | 



## basic_content_binary_post

> models::SaveBinaryResponse basic_content_binary_post(x_beam_gamertag, save_binary_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**save_binary_request** | Option<[**SaveBinaryRequest**](SaveBinaryRequest.md)> |  |  |

### Return type

[**models::SaveBinaryResponse**](SaveBinaryResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_binary_private_urls_post

> models::GetBinaryDownloadUrlsResponse basic_content_binary_private_urls_post(x_beam_gamertag, get_binary_download_urls_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**get_binary_download_urls_request** | Option<[**GetBinaryDownloadUrlsRequest**](GetBinaryDownloadUrlsRequest.md)> |  |  |

### Return type

[**models::GetBinaryDownloadUrlsResponse**](GetBinaryDownloadUrlsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_content_get

> models::ContentOrText basic_content_content_get(content_id, version, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_id** | **String** |  | [required] |
**version** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ContentOrText**](ContentOrText.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_localizations_delete

> models::CommonResponse basic_content_localizations_delete(x_beam_gamertag, delete_localization_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**delete_localization_request** | Option<[**DeleteLocalizationRequest**](DeleteLocalizationRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_localizations_get

> models::GetLocalizationsResponse basic_content_localizations_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::GetLocalizationsResponse**](GetLocalizationsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_localizations_put

> models::CommonResponse basic_content_localizations_put(x_beam_gamertag, put_localizations_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**put_localizations_request** | Option<[**PutLocalizationsRequest**](PutLocalizationsRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_checksum_get

> models::ContentBasicManifestChecksum basic_content_manifest_checksum_get(x_beam_gamertag, id, uid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**id** | Option<**String**> | ID of the content manifest |  |
**uid** | Option<**String**> | UID of the content manifest |  |

### Return type

[**models::ContentBasicManifestChecksum**](ContentBasicManifestChecksum.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_checksums_get

> models::ContentBasicManifestChecksums basic_content_manifest_checksums_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ContentBasicManifestChecksums**](ContentBasicManifestChecksums.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_diffs_get

> models::GetManifestDiffsResponse basic_content_manifest_diffs_get(manifest_id, x_beam_gamertag, from_uid, to_uid, offset, from_date, to_date, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manifest_id** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**from_uid** | Option<**String**> |  |  |
**to_uid** | Option<**String**> |  |  |
**offset** | Option<**i32**> |  |  |
**from_date** | Option<**i64**> |  |  |
**to_date** | Option<**i64**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::GetManifestDiffsResponse**](GetManifestDiffsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_exact_get

> models::ContentBasicManifest basic_content_manifest_exact_get(uid, x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ContentBasicManifest**](ContentBasicManifest.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_get

> models::ContentBasicManifest basic_content_manifest_get(x_beam_gamertag, id, uid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**id** | Option<**String**> | ID of the content manifest |  |
**uid** | Option<**String**> | UID of the content manifest |  |

### Return type

[**models::ContentBasicManifest**](ContentBasicManifest.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_history_get

> models::GetManifestHistoryResponse basic_content_manifest_history_get(x_beam_gamertag, id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**id** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::GetManifestHistoryResponse**](GetManifestHistoryResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_post

> models::SaveManifestResponse basic_content_manifest_post(x_beam_gamertag, save_manifest_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**save_manifest_request** | Option<[**SaveManifestRequest**](SaveManifestRequest.md)> |  |  |

### Return type

[**models::SaveManifestResponse**](SaveManifestResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_private_get

> models::ClientManifestResponse basic_content_manifest_private_get(x_beam_gamertag, id, uid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**id** | Option<**String**> | ID of the content manifest |  |
**uid** | Option<**String**> | UID of the content manifest |  |

### Return type

[**models::ClientManifestResponse**](ClientManifestResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_private_json_get

> models::ClientManifestJsonResponse basic_content_manifest_private_json_get(x_beam_gamertag, id, uid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**id** | Option<**String**> | Content ID of the content manifest |  |
**uid** | Option<**String**> | UID of the content manifest |  |

### Return type

[**models::ClientManifestJsonResponse**](ClientManifestJsonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_public_get

> models::ClientManifestResponse basic_content_manifest_public_get(x_beam_gamertag, id, uid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**id** | Option<**String**> | ID of the content manifest |  |
**uid** | Option<**String**> | UID of the content manifest |  |

### Return type

[**models::ClientManifestResponse**](ClientManifestResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_public_json_get

> models::ClientManifestJsonResponse basic_content_manifest_public_json_get(x_beam_gamertag, id, uid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**id** | Option<**String**> | Content ID of the content manifest |  |
**uid** | Option<**String**> | UID of the content manifest |  |

### Return type

[**models::ClientManifestJsonResponse**](ClientManifestJsonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_pull_post

> models::ContentBasicManifest basic_content_manifest_pull_post(x_beam_gamertag, pull_manifest_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**pull_manifest_request** | Option<[**PullManifestRequest**](PullManifestRequest.md)> |  |  |

### Return type

[**models::ContentBasicManifest**](ContentBasicManifest.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifest_repeat_put

> models::CommonResponse basic_content_manifest_repeat_put(x_beam_gamertag, repeat_manifest_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**repeat_manifest_request** | Option<[**RepeatManifestRequest**](RepeatManifestRequest.md)> |  |  |

### Return type

[**models::CommonResponse**](CommonResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifests_archive_post

> models::EmptyResponse basic_content_manifests_archive_post(x_beam_gamertag, archive_or_unarchive_manifests_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**archive_or_unarchive_manifests_request** | Option<[**ArchiveOrUnarchiveManifestsRequest**](ArchiveOrUnarchiveManifestsRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifests_get

> models::ContentBasicGetManifestsResponse basic_content_manifests_get(x_beam_gamertag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |

### Return type

[**models::ContentBasicGetManifestsResponse**](ContentBasicGetManifestsResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifests_pull_post

> models::ContentBasicManifestChecksums basic_content_manifests_pull_post(x_beam_gamertag, pull_all_manifests_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**pull_all_manifests_request** | Option<[**PullAllManifestsRequest**](PullAllManifestsRequest.md)> |  |  |

### Return type

[**models::ContentBasicManifestChecksums**](ContentBasicManifestChecksums.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_manifests_unarchive_post

> models::EmptyResponse basic_content_manifests_unarchive_post(x_beam_gamertag, archive_or_unarchive_manifests_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**archive_or_unarchive_manifests_request** | Option<[**ArchiveOrUnarchiveManifestsRequest**](ArchiveOrUnarchiveManifestsRequest.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_post

> models::SaveContentResponse basic_content_post(x_beam_gamertag, save_content_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**save_content_request** | Option<[**SaveContentRequest**](SaveContentRequest.md)> |  |  |

### Return type

[**models::SaveContentResponse**](SaveContentResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_content_text_post

> models::SaveTextResponse basic_content_text_post(x_beam_gamertag, save_text_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**save_text_request** | Option<[**SaveTextRequest**](SaveTextRequest.md)> |  |  |

### Return type

[**models::SaveTextResponse**](SaveTextResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

