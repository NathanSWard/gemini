use chrono::{serde::ts_milliseconds, DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::symbol::Symbol;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "result")]
pub enum Result {
    Success {
        auction_price: Decimal,
        auction_quantity: Decimal,
    },
    Failure,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AuctionData {
    pub symbol: Symbol,
    #[serde(rename = "time_ms", with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub highest_bid_price: Option<Decimal>,
    pub lowest_ask_price: Option<Decimal>,
    pub collar_price: Decimal,
    #[serde(flatten)]
    pub result: Result,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AuctionIndicative {
    #[serde(flatten)]
    pub data: AuctionData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AuctionResult {
    #[serde(flatten)]
    pub data: AuctionData,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Auction {
    #[serde(rename = "auction_indicative")]
    Indicative(AuctionIndicative),
    #[serde(rename = "auction_result")]
    Result(AuctionResult),
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

    const AUCTION_INDICATIVE_SUCCESS: &str = r#"{
        "type": "auction_indicative",
        "symbol": "ETHUSD",
        "result": "success",
        "time_ms": 1510865640000,
        "highest_bid_price": "1.2",
        "lowest_ask_price": "2.3",
        "collar_price": "4.5",
        "auction_price": "6.7",
        "auction_quantity": "8.9"
    }"#;

    const AUCTION_INDICATIVE_FAILURE: &str = r#"{
        "type": "auction_indicative",
        "symbol": "ETHUSD",
        "result": "failure",
        "time_ms": 1510865640000,
        "collar_price": "1.2",
        "auction_price": "0",
        "auction_quantity": "0"
    }"#;

    const AUCTION_OUTCOME_SUCCESS: &str = r#"{
        "type": "auction_result",
        "symbol": "ETHUSD",
        "result": "success",
        "time_ms": 1510866000000,
        "highest_bid_price": "1.2",
        "lowest_ask_price": "2.3",
        "collar_price": "4.5",
        "auction_price": "6.7",
        "auction_quantity": "8.9"
    }"#;

    const AUCTION_OUTCOME_FAILURE: &str = r#"{
        "type": "auction_result",
        "symbol": "ETHUSD",
        "result": "failure",
        "time_ms": 1510866000000,
        "collar_price": "1.2",
        "auction_price": "0",
        "auction_quantity": "0"
    }"#;

    #[test]
    fn test_auction_indicative_deserialize() {
        let success =
            serde_json::from_str::<AuctionIndicative>(AUCTION_INDICATIVE_SUCCESS).unwrap();

        assert!(matches!(success.data.symbol, Symbol::ETHUSD));
        assert_eq!(success.data.timestamp.timestamp_millis(), 1510865640000);
        assert_eq!(success.data.highest_bid_price, Some(dec!(1.2)));
        assert_eq!(success.data.lowest_ask_price, Some(dec!(2.3)));
        assert_eq!(success.data.collar_price, dec!(4.5));
        assert!(matches!(success.data.result, Result::Success {
            auction_price,
            auction_quantity,
        } if auction_price == dec!(6.7) && auction_quantity == dec!(8.9)));

        let failure =
            serde_json::from_str::<AuctionIndicative>(AUCTION_INDICATIVE_FAILURE).unwrap();

        assert!(matches!(failure.data.symbol, Symbol::ETHUSD));
        assert_eq!(failure.data.timestamp.timestamp_millis(), 1510865640000);
        assert!(failure.data.highest_bid_price.is_none());
        assert!(failure.data.lowest_ask_price.is_none());
        assert_eq!(failure.data.collar_price, dec!(1.2));
        assert!(matches!(failure.data.result, Result::Failure));
    }

    #[test]
    fn test_auction_result_deserialize() {
        let success = serde_json::from_str::<AuctionResult>(AUCTION_OUTCOME_SUCCESS).unwrap();

        assert!(matches!(success.data.symbol, Symbol::ETHUSD));
        assert_eq!(success.data.timestamp.timestamp_millis(), 1510866000000);
        assert_eq!(success.data.highest_bid_price, Some(dec!(1.2)));
        assert_eq!(success.data.lowest_ask_price, Some(dec!(2.3)));
        assert_eq!(success.data.collar_price, dec!(4.5));
        assert!(matches!(success.data.result, Result::Success {
            auction_price,
            auction_quantity,
        } if auction_price == dec!(6.7) && auction_quantity == dec!(8.9)));

        let failure = serde_json::from_str::<AuctionResult>(AUCTION_OUTCOME_FAILURE).unwrap();

        assert!(matches!(failure.data.symbol, Symbol::ETHUSD));
        assert_eq!(failure.data.timestamp.timestamp_millis(), 1510866000000);
        assert!(failure.data.highest_bid_price.is_none());
        assert!(failure.data.lowest_ask_price.is_none());
        assert_eq!(failure.data.collar_price, dec!(1.2));
        assert!(matches!(failure.data.result, Result::Failure));
    }

    #[test]
    fn test_auction_deserialization() {
        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_INDICATIVE_SUCCESS).unwrap(),
            Auction::Indicative(AuctionIndicative {
                data: AuctionData {
                    result: Result::Success { .. },
                    ..
                }
            })
        ));

        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_INDICATIVE_FAILURE).unwrap(),
            Auction::Indicative(AuctionIndicative {
                data: AuctionData {
                    result: Result::Failure,
                    ..
                }
            })
        ));

        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_OUTCOME_SUCCESS).unwrap(),
            Auction::Result(AuctionResult {
                data: AuctionData {
                    result: Result::Success { .. },
                    ..
                }
            })
        ));

        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_OUTCOME_FAILURE).unwrap(),
            Auction::Result(AuctionResult {
                data: AuctionData {
                    result: Result::Failure,
                    ..
                }
            })
        ));
    }
}
