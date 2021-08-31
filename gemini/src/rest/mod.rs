pub mod auction;
pub mod auction_history;
pub mod candles;
pub mod client;
pub mod order_book;
pub mod price_feed;
pub mod ticker;
pub mod trade_history;

pub trait HttpRequest: Sized {
    type Response: serde::de::DeserializeOwned;
    const METHOD: reqwest::Method;

    fn url(self, url: reqwest::Url) -> reqwest::Url;
}
