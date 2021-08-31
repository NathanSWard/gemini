use rust_decimal::Decimal;
use serde::Deserialize;

use crate::symbol::Symbol;

use super::HttpRequest;

#[derive(Deserialize, Debug, Clone)]
pub struct PriceFeed {
    #[serde(rename = "pair")]
    pub symbol: Symbol,
    pub price: Decimal,
    #[serde(rename = "percentChange24h")]
    pub percent_change_24h: Decimal,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct PriceFeeds {
    pub feeds: Vec<PriceFeed>,
}

pub struct PriceFeedsHttpRequest;

impl HttpRequest for PriceFeedsHttpRequest {
    type Response = PriceFeeds;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn url(self, mut url: reqwest::Url) -> reqwest::Url {
        url.path_segments_mut().unwrap().extend(["v1", "pricefeed"]);
        url
    }
}
