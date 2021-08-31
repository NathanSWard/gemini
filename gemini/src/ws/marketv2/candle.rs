use crate::symbol::Symbol;
use serde::Deserialize;

pub use crate::common::Candle;

/*
type 	string 	candles_1m_updates, candles_5m_updates, etc.
symbol 	string 	BTCUSD, etc.
candles 	Array of Arrays (TOHLCV) 	Changes to order book
-- -- time 	long 	milliseconds
-- -- open 	decimal 	Open price
-- -- high 	decimal 	High price
-- -- low 	decimal 	Low price
-- -- close 	decimal 	Close price
-- -- volume 	decimal 	Volume
*/

#[derive(Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CandleType {
    #[serde(rename = "candles_1m_updates")]
    Minute1,
    #[serde(rename = "candles_5m_updates")]
    Minute5,
    #[serde(rename = "candles_15m_updates")]
    Minute15,
    #[serde(rename = "candles_30m_updates")]
    Minute30,
    #[serde(rename = "candles_1h_updates")]
    Hour1,
    #[serde(rename = "candles_6h_updates")]
    Hour6,
    #[serde(rename = "candles_1d_updates")]
    Day1,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Candles {
    #[serde(rename = "type")]
    pub candle_type: CandleType,
    pub symbol: Symbol,
    #[serde(rename = "changes")]
    pub candles: Vec<Candle>,
}
