use crate::{rest::HttpRequest, symbol::Symbol};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Indicative,
    Auction,
}

#[derive(Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase", tag = "auction_result")]
pub enum AuctionResult {
    Success {
        auction_price: Decimal,
        auction_quantity: Decimal,
    },
    Failure,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Auction {
    #[serde(rename = "timestampms", with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,

    pub auction_id: u32,
    pub eid: u64,
    pub event_type: EventType,

    #[serde(flatten)]
    pub auction_result: AuctionResult,

    // Highest bid price from the continuous trading order book at the time of the auction event, if available.
    pub highest_bid_price: Option<Decimal>,

    // Lowest ask price from the continuous trading order book at the time of the auction event, if available.
    pub lowest_ask_price: Option<Decimal>,

    // The auction_price must be within plus or minus five percent of the collar price for result to be success.
    pub collar_price: Decimal,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct AuctionHistory {
    auctions: Vec<Auction>,
}

#[derive(TypedBuilder)]
pub struct AuctionHistoryHttpRequest {
    symbol: Symbol,
    #[builder(default, setter(strip_option))]
    timestamp: Option<DateTime<Utc>>,
    #[builder(default, setter(strip_option))]
    limit_auction_results: Option<u32>,
    #[builder(default, setter(strip_option))]
    include_indicative: Option<bool>,
}

impl HttpRequest for AuctionHistoryHttpRequest {
    type Response = AuctionHistory;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn url(self, mut url: reqwest::Url) -> reqwest::Url {
        url.path_segments_mut()
            .unwrap()
            .extend(["v1", "auction", self.symbol.as_ref(), "history"]);

        if crate::any_some!(
            self.timestamp,
            self.limit_auction_results,
            self.include_indicative
        ) {
            let mut query = url.query_pairs_mut();
            let ser = serde_urlencoded::Serializer::new(&mut query);
            (
                self.timestamp.map(|ts| ("timestamp", ts)),
                self.limit_auction_results
                    .map(|limit| ("limit_auction_results", limit)),
                self.include_indicative
                    .map(|include| ("include_indicative", include)),
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
    use rust_decimal_macros::dec;

    #[test]
    fn test_auction_history_deserialization() {
        let json = r#"[
            {
                "auction_id": 3,
                "auction_price": "1.1",
                "auction_quantity": "2.2",
                "eid": 4066,
                "highest_bid_price": "3.3",
                "lowest_ask_price": "4.4",
                "collar_price": "5.5",
                "auction_result": "success",
                "timestamp": 1471902531,
                "timestampms": 1471902531225,
                "event_type": "auction"
            },
            {
                "auction_id": 3,
                "auction_price": "0",
                "auction_quantity": "0",
                "eid": 3920,
                "collar_price": "1.1",
                "auction_result": "failure",
                "timestamp": 1471902471,
                "timestampms": 1471902471225,
                "event_type": "indicative"
            }
        ]"#;

        let history = serde_json::from_str::<AuctionHistory>(json).unwrap();

        assert_eq!(2, history.auctions.len());

        let success = &history.auctions[0];
        assert_eq!(success.auction_id, 3);
        assert_eq!(success.eid, 4066);
        assert_eq!(success.highest_bid_price, Some(dec!(3.3)));
        assert_eq!(success.lowest_ask_price, Some(dec!(4.4)));
        assert_eq!(success.collar_price, dec!(5.5));
        assert_eq!(success.timestamp.timestamp_millis(), 1471902531225);
        assert_eq!(success.event_type, EventType::Auction);
        if let AuctionResult::Success {
            auction_price,
            auction_quantity,
        } = success.auction_result
        {
            assert_eq!(auction_price, dec!(1.1));
            assert_eq!(auction_quantity, dec!(2.2));
        } else {
            panic!("auction_result should be a success");
        }

        let failure = &history.auctions[1];
        assert_eq!(failure.auction_id, 3);
        assert_eq!(failure.eid, 3920);
        assert_eq!(failure.highest_bid_price, None);
        assert_eq!(failure.lowest_ask_price, None);
        assert_eq!(failure.collar_price, dec!(1.1));
        assert_eq!(failure.timestamp.timestamp_millis(), 1471902471225);
        assert_eq!(failure.event_type, EventType::Indicative);
        if let AuctionResult::Success { .. } = failure.auction_result {
            panic!("auction_result should be a failure");
        }
    }
}
