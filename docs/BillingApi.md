# \BillingApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_billing_portal_session_post**](BillingApi.md#api_billing_portal_session_post) | **POST** /api/billing/portal-session | 



## api_billing_portal_session_post

> models::PortalSessionResponse api_billing_portal_session_post(x_beam_gamertag, x_beam_timeout)


Create a Stripe Customer Portal session so the customer can manage payment methods and subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_beam_gamertag** | Option<**String**> | Override the playerId of the requester. This is only necessary when not using a JWT bearer token. |  |
**x_beam_timeout** | Option<**i32**> | Set the request timeout in seconds. Defaults to 10 seconds. |  |

### Return type

[**models::PortalSessionResponse**](PortalSessionResponse.md)

### Authorization

[auth](../README.md#auth), [scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

