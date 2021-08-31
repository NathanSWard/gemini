use chrono::{serde::ts_milliseconds, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{rest::HttpRequest, symbol::Symbol};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TradeType {
    Buy,
    Sell,
    Auction,
    Block,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(from = "&str")]
pub struct Gemini;

impl From<&str> for Gemini {
    fn from(s: &str) -> Self {
        debug_assert!(s == "gemini");
        Self
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TradeHistory {
    #[serde(rename = "timestampms", with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub tid: u32,
    pub price: Decimal,
    pub amount: Decimal,
    pub exchange: Gemini,
    #[serde(rename = "type")]
    pub ty: TradeType,
    pub broken: bool,
}

#[derive(TypedBuilder)]
pub struct TradeHistoryHttpRequest {
    symbol: Symbol,
    #[builder(default, setter(strip_option))]
    timestamp: Option<DateTime<Utc>>,
    #[builder(default, setter(strip_option))]
    limit_trades: Option<u32>,
    #[builder(default, setter(strip_option))]
    include_breaks: Option<bool>,
}

impl HttpRequest for TradeHistoryHttpRequest {
    type Response = TradeHistory;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn url(self, mut url: reqwest::Url) -> reqwest::Url {
        url.path_segments_mut()
            .unwrap()
            .extend(["v1", "trades", self.symbol.as_ref()]);

        if crate::any_some!(self.timestamp, self.limit_trades, self.include_breaks) {
            let mut query = url.query_pairs_mut();
            let ser = serde_urlencoded::Serializer::new(&mut query);
            (
                self.timestamp.map(|ts| ("timestamp", ts)),
                self.limit_trades.map(|limit| ("limit_trades", limit)),
                self.include_breaks.map(|breaks| ("include_breaks", breaks)),
            )
                .serialize(ser)
                .unwrap();
        }

        url
    }
}

#[cfg(test)]
mod test {
    use reqwest::Url;

    use super::*;

    #[test]
    fn test_trade_request_url() {
        let req = TradeHistoryHttpRequest::builder()
            .symbol(Symbol::BTCUSD)
            .limit_trades(42)
            .include_breaks(true)
            .build();

        let url = req.url(Url::parse("https://domain.com").unwrap());
        let expected =
            Url::parse("https://domain.com/v1/trades/btcusd?limit_trades=42&include_breaks=true")
                .unwrap();

        assert_eq!(url, expected);
    }
}
