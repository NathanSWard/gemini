use crate::{rest::HttpRequest, symbol::Symbol};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub mod v1 {
    use super::*;
    use std::collections::HashMap;

    #[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PriceSymbol {
        USD,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum QuantitySymbol {
        BTC,
    }

    #[derive(Debug, Clone)]
    pub struct Pair<A, B> {
        a: A,
        b: B,
    }

    #[derive(Default)]
    struct PairVisitor<A, B> {
        pub(crate) _phantom: std::marker::PhantomData<(A, B)>,
    }

    impl<'de, A, B> serde::de::Visitor<'de> for PairVisitor<A, B>
    where
        A: Deserialize<'de>,
        B: Deserialize<'de>,
    {
        type Value = Pair<A, B>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "A single key-value pair")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            map.next_entry::<A, B>()?
                .map(|(a, b)| Pair { a, b })
                .ok_or_else(|| {
                    serde::de::Error::custom("`Pair` must contain a single key-value pair")
                })
        }
    }

    impl<'de, A, B> serde::Deserialize<'de> for Pair<A, B>
    where
        A: Deserialize<'de>,
        B: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_map(PairVisitor::<A, B> {
                _phantom: std::marker::PhantomData,
            })
        }
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct Volume {
        #[serde(with = "ts_milliseconds")]
        pub timestamp: DateTime<Utc>,

        // TODO: OPTIMIZE!!!
        #[serde(flatten)]
        pub other: HashMap<String, Decimal>,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct Ticker {
        pub bid: Decimal,
        pub ask: Decimal,
        pub last: Decimal,
        pub volume: Volume,
    }

    #[derive(TypedBuilder)]
    pub struct TickerHttpRequest {
        symbol: Symbol,
    }

    impl HttpRequest for TickerHttpRequest {
        type Response = Ticker;

        const METHOD: Method = Method::GET;

        fn url(self, mut url: reqwest::Url) -> reqwest::Url {
            url.path_segments_mut()
                .unwrap()
                .extend(["v1", "pubticker", self.symbol.as_ref()]);
            url
        }
    }
}

pub mod v2 {
    use super::*;

    #[derive(Deserialize, Clone, Debug)]
    pub struct Ticker {
        pub symbol: Symbol,
        // Open price from 24 hours ago
        pub open: Decimal,
        // High price from 24 hours ago
        pub high: Decimal,
        // Low price from 24 hours ago
        pub low: Decimal,
        // Open price (most recent trade)
        pub close: Decimal,
        // Hourly prices descending for past 24 hours
        pub changes: [Decimal; 24],
        // Current best bid
        pub bid: Decimal,
        // Current best offer
        pub ask: Decimal,
    }

    #[derive(TypedBuilder)]
    pub struct TickerHttpRequest {
        symbol: Symbol,
    }

    impl HttpRequest for TickerHttpRequest {
        type Response = Ticker;

        const METHOD: Method = Method::GET;

        fn url(self, mut url: reqwest::Url) -> reqwest::Url {
            url.path_segments_mut()
                .unwrap()
                .extend(["v2", "ticker", self.symbol.as_ref()]);
            url
        }
    }
}

#[cfg(test)]
mod test {
    use crate::symbol::Symbol;
    use rust_decimal_macros::dec;

    mod v1 {
        use super::super::v1::Ticker;
        use super::*;

        #[test]
        fn test_ticker_v1_deserialize() {
            let json = r#"{
                "ask": "977.59",
                "bid": "977.35",
                "last": "977.65",
                "volume": {
                    "BTC": "2210.505328803",
                    "USD": "2135477.463379586263",
                    "timestamp": 1483018200000
                }
            }"#;

            let ticker = serde_json::from_str::<Ticker>(json).unwrap();

            assert_eq!(ticker.ask, dec!(977.59));
            assert_eq!(ticker.bid, dec!(977.35));
            assert_eq!(ticker.last, dec!(977.65));
            assert_eq!(ticker.volume.timestamp.timestamp_millis(), 1483018200000);

            assert_eq!(ticker.volume.other.get("BTC"), Some(&dec!(2210.505328803)));
            assert_eq!(
                ticker.volume.other.get("USD"),
                Some(&dec!(2135477.463379586263))
            );
        }
    }

    mod v2 {
        use super::super::v2::Ticker;
        use super::*;

        #[test]
        fn test_ticker_v2_deserialization() {
            let json = r#"{
            "symbol": "btcusd",
            "open": "9121.76",
            "high": "9440.66",
            "low": "9106.51",
            "close": "9347.66",
            "changes": [
              "1.00",
              "1.01",
              "1.02",
              "1.03",
              "1.04",
              "1.05",
              "1.06",
              "1.07",
              "1.08",
              "1.09",
              "1.10",
              "1.11",
              "1.12",
              "1.13",
              "1.14",
              "1.15",
              "1.16",
              "1.17",
              "1.18",
              "1.19",
              "1.20",
              "1.21",
              "1.22",
              "1.23"
            ],
            "bid": "9345.70",
            "ask": "9347.67"
          }"#;

            let ticker = serde_json::from_str::<Ticker>(json).unwrap();

            assert_eq!(ticker.symbol, Symbol::BTCUSD);
            assert_eq!(ticker.open, dec!(9121.76));
            assert_eq!(ticker.high, dec!(9440.66));
            assert_eq!(ticker.low, dec!(9106.51));
            assert_eq!(ticker.close, dec!(9347.66));
            assert_eq!(ticker.bid, dec!(9345.70));
            assert_eq!(ticker.ask, dec!(9347.67));

            let mut init = dec!(0.99);
            let changes = (0..24)
                .map(|_| {
                    init += dec!(0.01);
                    init
                })
                .collect::<Vec<_>>();

            assert!(ticker.changes.iter().eq(changes.iter()));
        }
    }
}
