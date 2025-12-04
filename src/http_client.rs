//! This module contains an abstraction for HTTP clients as well as friendly-named re-exports of
//! client types that implement this trait.

use std::{future::Future, pin::Pin};

use bytes::BufMut;
use ruma::api::{
    AppserviceUserIdentity, OutgoingRequest,
    auth_scheme::{AuthScheme, SendAccessToken},
    path_builder::PathBuilder,
};

use crate::{ResponseError, ResponseResult};

#[cfg(feature = "hyper")]
mod hyper;
#[cfg(feature = "reqwest")]
mod reqwest;

#[cfg(feature = "hyper")]
pub use self::hyper::Hyper;
#[cfg(feature = "hyper-native-tls")]
pub use self::hyper::HyperNativeTls;
#[cfg(feature = "hyper-rustls")]
pub use self::hyper::HyperRustls;
#[cfg(feature = "reqwest")]
pub use self::reqwest::Reqwest;

/// An HTTP client that can be used to send requests to a Matrix homeserver.
pub trait HttpClient: Sync {
    /// The type to use for `try_into_http_request`.
    type RequestBody: AsRef<[u8]> + Default + BufMut + Send;

    /// The type to use for `try_from_http_response`.
    type ResponseBody: AsRef<[u8]>;

    /// The error type for the `send_request` function.
    type Error: Send + Unpin;

    /// Send an `http::Request` to get back an `http::Response`.
    fn send_http_request(
        &self,
        req: http::Request<Self::RequestBody>,
    ) -> impl Future<Output = Result<http::Response<Self::ResponseBody>, Self::Error>> + Send;
}

/// An HTTP client that has a default configuration.
pub trait DefaultConstructibleHttpClient: HttpClient {
    /// Creates a new HTTP client with default configuration.
    fn default() -> Self;
}

/// Convenience functionality on top of `HttpClient`.
///
/// If you want to build your own matrix client type instead of using `ruma_client::Client`, this
/// trait should make that relatively easy.
pub trait HttpClientExt: HttpClient {
    /// Send a strongly-typed matrix request to get back a strongly-typed response.
    fn send_matrix_request<'a, R>(
        &'a self,
        homeserver_url: &str,
        access_token: SendAccessToken<'a>,
        path_builder_input: <R::PathBuilder as PathBuilder>::Input<'_>,
        request: R,
    ) -> Pin<Box<dyn Future<Output = ResponseResult<Self, R>> + 'a + Send>>
    where
        R: OutgoingRequest,
        R::Authentication: AuthScheme<Input<'a> = SendAccessToken<'a>>,
    {
        self.send_customized_matrix_request(
            homeserver_url,
            access_token,
            path_builder_input,
            request,
            |_| Ok(()),
        )
    }

    /// Turn a strongly-typed matrix request into an `http::Request`, customize it and send it to
    /// get back a strongly-typed response.
    fn send_customized_matrix_request<'a, R, F>(
        &'a self,
        homeserver_url: &str,
        access_token: SendAccessToken<'a>,
        path_builder_input: <R::PathBuilder as PathBuilder>::Input<'_>,
        request: R,
        customize: F,
    ) -> Pin<Box<dyn Future<Output = ResponseResult<Self, R>> + 'a + Send>>
    where
        R: OutgoingRequest,
        R::Authentication: AuthScheme<Input<'a> = SendAccessToken<'a>>,
        F: FnOnce(&mut http::Request<Self::RequestBody>) -> Result<(), ResponseError<Self, R>>,
    {
        Box::pin(crate::send_customized_request(
            self,
            homeserver_url,
            access_token,
            path_builder_input,
            request,
            customize,
        ))
    }

    /// Turn a strongly-typed matrix request into an `http::Request`, add `user_id` and/or
    /// `device_id` query parameters to it and send it to get back a strongly-typed response.
    ///
    /// This method is meant to be used by application services when interacting with the
    /// client-server API.
    fn send_matrix_request_as<'a, R>(
        &'a self,
        homeserver_url: &str,
        access_token: SendAccessToken<'a>,
        path_builder_input: <R::PathBuilder as PathBuilder>::Input<'_>,
        identity: AppserviceUserIdentity<'a>,
        request: R,
    ) -> Pin<Box<dyn Future<Output = ResponseResult<Self, R>> + 'a>>
    where
        R: OutgoingRequest,
        R::Authentication: AuthScheme<Input<'a> = SendAccessToken<'a>>,
    {
        self.send_customized_matrix_request(
            homeserver_url,
            access_token,
            path_builder_input,
            request,
            |uri| Ok(identity.maybe_add_to_uri(uri.uri_mut())?),
        )
    }
}

impl<T: HttpClient> HttpClientExt for T {}

#[doc(hidden)]
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)]
pub struct Dummy;

impl HttpClient for Dummy {
    type RequestBody = Vec<u8>;
    type ResponseBody = Vec<u8>;
    type Error = ();

    #[allow(clippy::diverging_sub_expression)]
    async fn send_http_request(
        &self,
        _req: http::Request<Self::RequestBody>,
    ) -> Result<http::Response<Self::ResponseBody>, Self::Error> {
        unimplemented!("this client only exists to allow doctests to compile")
    }
}

impl DefaultConstructibleHttpClient for Dummy {
    fn default() -> Self {
        Dummy
    }
}
