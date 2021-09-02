use std::ops::{Deref, DerefMut};

use async_tungstenite::{
    tokio::{connect_async, ConnectStream},
    tungstenite::{handshake::client::Response, Error},
    WebSocketStream,
};
use reqwest::Url;

pub type WssStream = WebSocketStream<ConnectStream>;

pub async fn connect_wss_with_request(
    url: Url,
    req: impl WssRequest,
) -> Result<(WebSocketStream<ConnectStream>, Response), Error> {
    connect_async(req.url(url)).await
}

pub async fn connect_wss(url: Url) -> Result<(WebSocketStream<ConnectStream>, Response), Error> {
    connect_async(url).await
}

pub trait WssRequest: Sized {
    type Response: serde::de::DeserializeOwned;

    fn url(self, url: Url) -> Url;
}
