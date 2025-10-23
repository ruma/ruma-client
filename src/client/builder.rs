use std::sync::{Arc, Mutex};

use ruma::api::{
    client::discovery::get_supported_versions, MatrixVersion, SendAccessToken, SupportedVersions,
};

use super::{Client, ClientData};
use crate::{DefaultConstructibleHttpClient, Error, HttpClient, HttpClientExt};

/// A [`Client`] builder.
///
/// This type can be used to construct a `Client` through a few method calls.
pub struct ClientBuilder {
    homeserver_url: Option<String>,
    is_appservice: Option<bool>,
    access_token: Option<String>,
    always_send_token: Option<bool>,
    supported_matrix_versions: Option<SupportedVersions>,
}

impl ClientBuilder {
    pub(super) fn new() -> Self {
        Self {
            homeserver_url: None,
            is_appservice: None,
            access_token: None,
            always_send_token: None,
            supported_matrix_versions: None,
        }
    }

    /// Set the homeserver URL.
    ///
    /// The homeserver URL must be set before calling [`build()`][Self::build] or
    /// [`http_client()`][Self::http_client].
    pub fn homeserver_url(self, url: String) -> Self {
        Self { homeserver_url: Some(url), ..self }
    }

    /// Set whether the client is for an application service.
    pub fn is_appservice(self, is_appservice: bool) -> Self {
        Self { is_appservice: Some(is_appservice), ..self }
    }

    /// Set the access token.
    pub fn access_token(self, access_token: Option<String>) -> Self {
        Self { access_token, ..self }
    }

    /// Set whether the client should always send the access token.
    /// This setting is ignored if the client is for an application service.
    pub fn always_send_token(self, always_send_token: bool) -> Self {
        Self { always_send_token: Some(always_send_token), ..self }
    }

    /// Set the supported Matrix versions.
    ///
    /// This method generally *shouldn't* be called. The [`build()`][Self::build] or
    /// [`http_client()`][Self::http_client] method will take care of doing a
    /// [`get_supported_versions`] request to find out about the supported versions.
    pub fn supported_matrix_versions(self, versions: SupportedVersions) -> Self {
        Self { supported_matrix_versions: Some(versions), ..self }
    }

    /// Finish building the [`Client`].
    ///
    /// Uses [`DefaultConstructibleHttpClient::default()`] to create an HTTP client instance.
    /// Unless the supported Matrix versions were manually set via
    /// [`supported_matrix_versions`][Self::supported_matrix_versions], this will do a
    /// [`get_supported_versions`] request to find out about the supported versions.
    pub async fn build<C>(self) -> Result<Client<C>, Error<C::Error, ruma::api::client::Error>>
    where
        C: DefaultConstructibleHttpClient,
    {
        self.http_client(C::default()).await
    }

    /// Set the HTTP client to finish building the [`Client`].
    ///
    /// Unless the supported Matrix versions were manually set via
    /// [`supported_matrix_versions`][Self::supported_matrix_versions], this will do a
    /// [`get_supported_versions`] request to find out about the supported versions.
    pub async fn http_client<C>(
        self,
        http_client: C,
    ) -> Result<Client<C>, Error<C::Error, ruma::api::client::Error>>
    where
        C: HttpClient,
    {
        let homeserver_url = self
            .homeserver_url
            .expect("homeserver URL has to be set prior to calling .build() or .http_client()");

        let supported_matrix_versions = match self.supported_matrix_versions {
            Some(versions) => versions,
            None => http_client
                .send_matrix_request(
                    &homeserver_url,
                    SendAccessToken::None,
                    &SupportedVersions {
                        versions: [MatrixVersion::V1_0].into(),
                        features: Default::default(),
                    },
                    get_supported_versions::Request::new(),
                )
                .await?
                .as_supported_versions(),
        };

        Ok(Client(Arc::new(ClientData {
            homeserver_url,
            http_client,
            is_appservice: self.is_appservice.unwrap_or(false),
            access_token: Mutex::new(self.access_token),
            always_send_token: self.always_send_token.unwrap_or(false),
            supported_matrix_versions,
        })))
    }
}
