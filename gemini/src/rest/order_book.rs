use std::num::NonZeroU32;

use crate::{rest::HttpRequest, symbol::Symbol};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Clone, Debug)]
pub struct Order {
    price: Decimal,
    amount: Decimal,
    // timestamp -> Unused
}

#[derive(Deserialize, Clone, Debug)]
pub struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(into = "u32")]
pub enum Limit {
    FullBook,
    Exact(NonZeroU32),
}

impl From<Limit> for u32 {
    fn from(limit: Limit) -> Self {
        match limit {
            Limit::FullBook => 0,
            Limit::Exact(n) => n.get(),
        }
    }
}

#[derive(TypedBuilder)]
pub struct OrderBookHttpRequest {
    symbol: Symbol,
    #[builder(default, setter(strip_option))]
    bids_limit: Option<Limit>,
    #[builder(default, setter(strip_option))]
    asks_limit: Option<Limit>,
}

impl HttpRequest for OrderBookHttpRequest {
    type Response = OrderBook;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn url(self, mut url: reqwest::Url) -> reqwest::Url {
        url.path_segments_mut()
            .unwrap()
            .extend(["v1", "book", self.symbol.as_ref()]);

        if crate::any_some!(self.bids_limit, self.asks_limit) {
            let mut query = url.query_pairs_mut();
            let ser = serde_urlencoded::Serializer::new(&mut query);
            (
                self.bids_limit.map(|limit| ("limit_bids", limit)),
                self.asks_limit.map(|limit| ("limit_asks", limit)),
            )
                .serialize(ser)
                .unwrap();
        }

        url
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_book_request_url_impl(
        bids: Option<Limit>,
        asks: Option<Limit>,
        query: impl Into<Option<&'static str>>,
    ) {
        let req = OrderBookHttpRequest {
            symbol: Symbol::BTCUSD,
            bids_limit: bids,
            asks_limit: asks,
        };

        let url = reqwest::Url::parse("https://domain.com").unwrap();
        let url = req.url(url);

        let expected = match query.into() {
            Some(query) => format!("https://domain.com/v1/book/btcusd{}", query),
            None => "https://domain.com/v1/book/btcusd".to_owned(),
        };

        assert_eq!(url, reqwest::Url::parse(&expected).unwrap());
    }

    #[test]
    fn test_book_request_url() {
        test_book_request_url_impl(None, None, None);
        test_book_request_url_impl(None, Some(Limit::FullBook), "?limit_asks=0");
        test_book_request_url_impl(
            Some(Limit::Exact(NonZeroU32::new(1).unwrap())),
            None,
            "?limit_bids=1",
        );
        test_book_request_url_impl(
            Some(Limit::Exact(NonZeroU32::new(42).unwrap())),
            Some(Limit::FullBook),
            "?limit_bids=42&limit_asks=0",
        );
    }
}
