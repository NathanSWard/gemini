use crate::{rest::HttpRequest, symbol::Symbol};
use reqwest::{Method, Url};
use serde::Deserialize;
use strum_macros::{AsRefStr, IntoStaticStr};
use typed_builder::TypedBuilder;

pub use crate::common::Candle;

#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Candles {
    pub candles: Vec<Candle>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, AsRefStr, IntoStaticStr)]
pub enum TimeRange {
    #[strum(serialize = "1m")]
    Minute1,
    #[strum(serialize = "5m")]
    Minute5,
    #[strum(serialize = "15m")]
    Minute15,
    #[strum(serialize = "30m")]
    Minute30,
    #[strum(serialize = "1h")]
    Hour1,
    #[strum(serialize = "6h")]
    Hour6,
    #[strum(serialize = "1day")]
    Day1,
}

#[derive(TypedBuilder)]
pub struct CandleHttpRequest {
    symbol: Symbol,
    time_range: TimeRange,
}

impl HttpRequest for CandleHttpRequest {
    type Response = Candles;

    const METHOD: Method = Method::GET;

    fn url(self, mut url: Url) -> Url {
        url.path_segments_mut().unwrap().extend([
            "v2",
            "candles",
            self.symbol.as_ref(),
            self.time_range.as_ref(),
        ]);
        url
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    pub fn test_candle_deserialization() {
        let json = r#"[
            [
             1559755800000,
             7781.6,
             7820.23,
             7776.56,
             7819.39,
             34.7624802159
            ],
            [1559755800000,
            7781.6,
            7829.46,
            7776.56,
            7817.28,
            43.4228281059]
        ]"#;

        let candles = serde_json::from_str::<Candles>(json).unwrap();

        assert_eq!(2, candles.candles.len());

        let candle = candles.candles.first().unwrap();
        assert_eq!(candle.time.timestamp_millis(), 1559755800000);
        assert_eq!(candle.open, dec!(7781.6));
        assert_eq!(candle.high, dec!(7820.23));
        assert_eq!(candle.low, dec!(7776.56));
        assert_eq!(candle.close, dec!(7819.39));
        assert_eq!(candle.volume, dec!(34.7624802159));
    }
}
