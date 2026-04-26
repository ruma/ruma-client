# Changelog

## [unreleased]

Breaking changes:

- Upgrade reqwest to 0.13.
  - The `reqwest-*` cargo features were updated to match the changes upstream.
- Upgrade ruma to 0.15.0.
  - Bump the MSRV to 1.89.

## 0.17.0

Breaking changes:

- Upgrade ruma to 0.14.1.
  - The `send_request`, `send_request_as` and `send_customized_request` of
    `Client` now have stricter bounds for the request. These bounds are
    compatible with all requests from ruma-client-api and ruma-appservice-api.
  - `HttpRequest::RequestBuilder` has an extra `AsRef<[u8]>` bound.
  - Bump MSRV to 1.88
  - `Client::send_request_as()` and `HttpClientExt::send_matrix_request_as()`
    now take `AppserviceUserIdentity` instead of `&UserId`.

Improvements:

- `ClientBuilder` now has `token_mode()` which takes a `TokenMode` for
  correlation to `SendAccessToken` behavior.

## 0.16.0

Breaking changes:

- Upgrade ruma to 0.13.0.
  - `ClientBuilder::supported_matrix_versions()` now takes a `SupportedVersions`.

## 0.15.0

Upgrade `ruma-client-api` to 0.20.0.

## 0.14.0

No changes for this version

## 0.13.0

Breaking changes:

- Remove `isahc` feature

Improvements:

- Add `error_kind` accessor method to `Error<E, ruma_client_api::Error>`

## 0.12.0

No changes for this version

## 0.11.0

No changes for this version

## 0.10.0

Breaking changes:

- Upgrade dependencies

## 0.9.0

Breaking changes:

- Upgrade dependencies

## 0.8.0

Breaking changes:

- Upgrade dependencies
- The whole `Client` is now feature-gated (`client-api` feature).
  We may introduce a separate `FederationClient` and possibly other types like
  that in the future.

Improvements:

- Rewrite `Client` initialization and store server-supported Matrix versions in
  it, to determine whether to use stable, unstable or r0 paths for endpoints

## 0.7.0

Breaking changes:

- Upgrade dependencies

## 0.6.0

Breaking changes:

- Upgrade ruma-client-api to 0.11.0

## 0.5.0

Breaking changes:

- Make `Client` generic over the http client
- Make the ruma-client-api dependency optional
- Upgrade dependencies

Improvements:

- Add support for multiple HTTP clients
