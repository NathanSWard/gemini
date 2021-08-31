use crate::chrono::FromMilliseconds;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

type CandleRepr = (i64, Decimal, Decimal, Decimal, Decimal, Decimal);

#[derive(Deserialize, Debug, Clone)]
#[serde(from = "CandleRepr")]
pub struct Candle {
    pub time: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

impl From<CandleRepr> for Candle {
    fn from(repr: CandleRepr) -> Self {
        Self {
            time: DateTime::<Utc>::from_milliseconds(repr.0),
            open: repr.1,
            high: repr.2,
            low: repr.3,
            close: repr.4,
            volume: repr.5,
        }
    }
}
