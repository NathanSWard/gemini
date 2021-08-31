use chrono::{serde::ts_milliseconds, DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

crate::string_field_impl!(AuctionOpenTag, "auction_open");
crate::string_field_impl!(AuctionIndicativeTag, "auction_indicative");
crate::string_field_impl!(AuctionOutcomeTag, "auction_result");

#[derive(Deserialize, Debug, Clone)]
pub struct AuctionOpen {
    #[serde(rename = "type")]
    pub ty: AuctionOpenTag,
    #[serde(with = "ts_milliseconds")]
    pub auction_open_ms: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub auction_time_ms: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub first_indicative_ms: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub last_cancel_time_ms: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuctionData {
    pub eid: u32,
    #[serde(rename = "time_ms", with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub highest_bid_price: Option<Decimal>,
    pub lowest_ask_price: Option<Decimal>,
    pub collar_price: Decimal,
}

// TODO: move to common
#[derive(Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "result")]
pub enum AuctionIndicativeResult {
    Success { indicative_price: Decimal },
    Failure,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuctionIndicative {
    #[serde(rename = "type")]
    pub ty: AuctionIndicativeTag,

    #[serde(flatten)]
    pub result: AuctionIndicativeResult,
    pub indicative_quantity: Decimal,
    #[serde(flatten)]
    pub data: AuctionData,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase", tag = "result")]
pub enum AuctionOutcomeResult {
    Success {
        auction_price: Decimal,
        auction_quantity: Decimal,
    },
    Failure,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuctionOutcome {
    #[serde(rename = "type")]
    pub ty: AuctionOutcomeTag,
    #[serde(flatten)]
    pub result: AuctionOutcomeResult,
    #[serde(flatten)]
    pub data: AuctionData,
}


// TODO: maybe tag and make tagged/untagged variant versions
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Auction {
    Open(AuctionOpen),
    Indicative(AuctionIndicative),
    Outcome(AuctionOutcome),
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use rust_decimal_macros::dec;

    pub const AUCTION_TEST_STRINGS: &[&str] = &[AUCTION_OPEN, AUCTION_INDICATIVE_SUCCESS, AUCTION_INDICATIVE_FAILURE, AUCTION_OUTCOME_SUCCESS, AUCTION_OUTCOME_FAILURE];

    const AUCTION_OPEN: &str = r#"{
        "auction_open_ms": 1486591200000,
        "auction_time_ms": 1486674000000,
        "first_indicative_ms": 1486673400000,
        "last_cancel_time_ms": 1486673985000,
        "type": "auction_open"
    }"#;

    const AUCTION_INDICATIVE_SUCCESS: &str = r#"{
            "type": "auction_indicative",
            "eid": 2248762586,
            "result": "success",
            "time_ms": 1510865640000,
            "highest_bid_price": "7730.69",
            "lowest_ask_price": "7730.7",
            "collar_price": "7730.695",
            "indicative_price": "7750",
            "indicative_quantity": "45.43325086"
        }"#;

    const AUCTION_INDICATIVE_FAILURE: &str = r#"{
            "type": "auction_indicative",
            "eid": 2248762586,
            "result": "failure",
            "time_ms": 1510865640000,
            "collar_price": "7730.695",
            "indicative_price": "0",
            "indicative_quantity": "45.43325086"
        }"#;

    const AUCTION_OUTCOME_SUCCESS: &str = r#"{
            "type": "auction_result",
            "eid": 2248795680,
            "result": "success",
            "time_ms": 1510866000000,
            "highest_bid_price": "7769",
            "lowest_ask_price": "7769.01",
            "collar_price": "7769.005",
            "auction_price": "7763.23",
            "auction_quantity": "55.95"
        }"#;

    const AUCTION_OUTCOME_FAILURE: &str = r#"{
            "type": "auction_result",
            "eid": 2248795680,
            "result": "failure",
            "time_ms": 1510866000000,
            "collar_price": "7769.005",
            "auction_price": "0",
            "auction_quantity": "0"
        }"#;

    #[test]
    fn test_auction_open_deserialize() {
        let open = serde_json::from_str::<AuctionOpen>(AUCTION_OPEN).unwrap();

        assert_eq!(open.auction_open_ms.timestamp_millis(), 1486591200000);
        assert_eq!(open.auction_time_ms.timestamp_millis(), 1486674000000);
        assert_eq!(open.first_indicative_ms.timestamp_millis(), 1486673400000);
        assert_eq!(open.last_cancel_time_ms.timestamp_millis(), 1486673985000);
    }

    #[test]
    fn test_auction_indicative_deserialize() {
        let success =
            serde_json::from_str::<AuctionIndicative>(AUCTION_INDICATIVE_SUCCESS).unwrap();

        assert_eq!(success.data.eid, 2248762586);
        assert_eq!(success.data.timestamp.timestamp_millis(), 1510865640000);
        assert_eq!(success.data.highest_bid_price, Some(dec!(7730.69)));
        assert_eq!(success.data.lowest_ask_price, Some(dec!(7730.7)));
        assert_eq!(success.data.collar_price, dec!(7730.695));
        assert_eq!(success.indicative_quantity, dec!(45.43325086));
        assert!(matches!(success.result, 
            AuctionIndicativeResult::Success { indicative_price }
                if indicative_price == dec!(7750)));

        let failure =
            serde_json::from_str::<AuctionIndicative>(AUCTION_INDICATIVE_FAILURE).unwrap();

        assert_eq!(failure.data.eid, 2248762586);
        assert_eq!(failure.data.timestamp.timestamp_millis(), 1510865640000);
        assert!(failure.data.highest_bid_price.is_none());
        assert!(failure.data.lowest_ask_price.is_none());
        assert_eq!(failure.data.collar_price, dec!(7730.695));
        assert_eq!(failure.indicative_quantity, dec!(45.43325086));
        assert!(matches!(failure.result, AuctionIndicativeResult::Failure));
    }

    #[test]
    fn test_auction_outcome_deserialize() {
        let success = serde_json::from_str::<AuctionOutcome>(AUCTION_OUTCOME_SUCCESS).unwrap();

        assert_eq!(success.data.eid, 2248795680);
        assert_eq!(success.data.timestamp.timestamp_millis(), 1510866000000);
        assert_eq!(success.data.highest_bid_price, Some(dec!(7769)));
        assert_eq!(success.data.lowest_ask_price, Some(dec!(7769.01)));
        assert_eq!(success.data.collar_price, dec!(7769.005));
        assert!(matches!(success.result, 
            AuctionOutcomeResult::Success { auction_price, auction_quantity }
                if auction_price == dec!(7763.23) && auction_quantity == dec!(55.95)));

        let failure = serde_json::from_str::<AuctionOutcome>(AUCTION_OUTCOME_FAILURE).unwrap();

        assert_eq!(failure.data.eid, 2248795680);
        assert_eq!(failure.data.timestamp.timestamp_millis(), 1510866000000);
        assert!(failure.data.highest_bid_price.is_none());
        assert!(failure.data.lowest_ask_price.is_none());
        assert_eq!(failure.data.collar_price, dec!(7769.005));
        assert!(matches!(failure.result, AuctionOutcomeResult::Failure));
    }

    #[test]
    fn test_auction_enum_deserialize() {
        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_OPEN).unwrap(),
            Auction::Open(_)
        ));

        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_INDICATIVE_SUCCESS).unwrap(),
            Auction::Indicative(AuctionIndicative {
                result: AuctionIndicativeResult::Success { .. },
                ..
            })
        ));
        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_INDICATIVE_FAILURE).unwrap(),
            Auction::Indicative(AuctionIndicative {
                result: AuctionIndicativeResult::Failure,
                ..
            })
        ));

        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_OUTCOME_SUCCESS).unwrap(),
            Auction::Outcome(AuctionOutcome {
                result: AuctionOutcomeResult::Success { .. },
                ..
            })
        ));
        assert!(matches!(
            serde_json::from_str::<Auction>(AUCTION_OUTCOME_FAILURE).unwrap(),
            Auction::Outcome(AuctionOutcome {
                result: AuctionOutcomeResult::Failure,
                ..
            })
        ));
    }
}
