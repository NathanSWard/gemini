use std::ops::{Deref, DerefMut};

use async_tungstenite::{
    tokio::{connect_async, ConnectStream},
    tungstenite::{handshake::client::Response, Error},
    WebSocketStream,
};
use reqwest::Url;

pub async fn connect_wss_with_request(
    url: Url,
    req: impl WssRequest,
) -> Result<(WebSocketStream<ConnectStream>, Response), Error> {
    connect_async(req.url(url)).await
}

pub struct Client {
    inner: WebSocketStream<ConnectStream>,
}

impl Deref for Client {
    type Target = WebSocketStream<ConnectStream>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub async fn connect_wss(url: Url) -> Result<(Client, Response), Error> {
    connect_async(url)
        .await
        .map(|(wss, resp)| (Client { inner: wss }, resp))
}

pub trait WssRequest: Sized {
    type Response: serde::de::DeserializeOwned;

    fn url(self, url: Url) -> Url;
}
