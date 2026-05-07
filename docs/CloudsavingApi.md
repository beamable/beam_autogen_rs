# \CloudsavingApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_cloudsaving_data_commit_manifest_put**](CloudsavingApi.md#basic_cloudsaving_data_commit_manifest_put) | **PUT** /basic/cloudsaving/data/commitManifest | 
[**basic_cloudsaving_data_delete**](CloudsavingApi.md#basic_cloudsaving_data_delete) | **DELETE** /basic/cloudsaving/data | 
[**basic_cloudsaving_data_download_url_from_portal_post**](CloudsavingApi.md#basic_cloudsaving_data_download_url_from_portal_post) | **POST** /basic/cloudsaving/data/downloadURLFromPortal | 
[**basic_cloudsaving_data_download_url_post**](CloudsavingApi.md#basic_cloudsaving_data_download_url_post) | **POST** /basic/cloudsaving/data/downloadURL | 
[**basic_cloudsaving_data_move_from_portal_put**](CloudsavingApi.md#basic_cloudsaving_data_move_from_portal_put) | **PUT** /basic/cloudsaving/data/moveFromPortal | 
[**basic_cloudsaving_data_move_put**](CloudsavingApi.md#basic_cloudsaving_data_move_put) | **PUT** /basic/cloudsaving/data/move | 
[**basic_cloudsaving_data_replace_post**](CloudsavingApi.md#basic_cloudsaving_data_replace_post) | **POST** /basic/cloudsaving/data/replace | 
[**basic_cloudsaving_data_upload_url_from_portal_post**](CloudsavingApi.md#basic_cloudsaving_data_upload_url_from_portal_post) | **POST** /basic/cloudsaving/data/uploadURLFromPortal | 
[**basic_cloudsaving_data_upload_url_post**](CloudsavingApi.md#basic_cloudsaving_data_upload_url_post) | **POST** /basic/cloudsaving/data/uploadURL | 
[**basic_cloudsaving_get**](CloudsavingApi.md#basic_cloudsaving_get) | **GET** /basic/cloudsaving/ | 



## basic_cloudsaving_data_commit_manifest_put

> models::CloudsavingBasicManifest basic_cloudsaving_data_commit_manifest_put(x_beam_gamertag, upload_requests)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**upload_requests** | Option<[**UploadRequests**](UploadRequests.md)> |  |  |

### Return type

[**models::CloudsavingBasicManifest**](CloudsavingBasicManifest.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_data_delete

> models::EmptyResponse basic_cloudsaving_data_delete(x_beam_gamertag, object_requests)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**object_requests** | Option<[**ObjectRequests**](ObjectRequests.md)> |  |  |

### Return type

[**models::EmptyResponse**](EmptyResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_data_download_url_from_portal_post

> models::UrlsResponse basic_cloudsaving_data_download_url_from_portal_post(x_beam_gamertag, object_requests)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**object_requests** | Option<[**ObjectRequests**](ObjectRequests.md)> |  |  |

### Return type

[**models::UrlsResponse**](URLSResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_data_download_url_post

> models::UrlsResponse basic_cloudsaving_data_download_url_post(x_beam_gamertag, object_requests)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**object_requests** | Option<[**ObjectRequests**](ObjectRequests.md)> |  |  |

### Return type

[**models::UrlsResponse**](URLSResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_data_move_from_portal_put

> models::CloudsavingBasicManifest basic_cloudsaving_data_move_from_portal_put(x_beam_gamertag, player_basic_cloud_data_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**player_basic_cloud_data_request** | Option<[**PlayerBasicCloudDataRequest**](PlayerBasicCloudDataRequest.md)> |  |  |

### Return type

[**models::CloudsavingBasicManifest**](CloudsavingBasicManifest.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_data_move_put

> models::CloudsavingBasicManifest basic_cloudsaving_data_move_put(x_beam_gamertag, player_basic_cloud_data_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**player_basic_cloud_data_request** | Option<[**PlayerBasicCloudDataRequest**](PlayerBasicCloudDataRequest.md)> |  |  |

### Return type

[**models::CloudsavingBasicManifest**](CloudsavingBasicManifest.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_data_replace_post

> models::CloudsavingBasicManifest basic_cloudsaving_data_replace_post(x_beam_gamertag, replace_objects_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**replace_objects_request** | Option<[**ReplaceObjectsRequest**](ReplaceObjectsRequest.md)> |  |  |

### Return type

[**models::CloudsavingBasicManifest**](CloudsavingBasicManifest.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_data_upload_url_from_portal_post

> models::UrlsResponse basic_cloudsaving_data_upload_url_from_portal_post(x_beam_gamertag, upload_requests_from_portal)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**upload_requests_from_portal** | Option<[**UploadRequestsFromPortal**](UploadRequestsFromPortal.md)> |  |  |

### Return type

[**models::UrlsResponse**](URLSResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_data_upload_url_post

> models::UrlsResponse basic_cloudsaving_data_upload_url_post(x_beam_gamertag, upload_requests)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**upload_requests** | Option<[**UploadRequests**](UploadRequests.md)> |  |  |

### Return type

[**models::UrlsResponse**](URLSResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_cloudsaving_get

> models::CloudsavingBasicManifest basic_cloudsaving_get(x_beam_gamertag, player_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the Gamer Tag of the player. This is generally inferred by the auth token. |  |
**player_id** | Option<**i64**> |  |  |

### Return type

[**models::CloudsavingBasicManifest**](CloudsavingBasicManifest.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

