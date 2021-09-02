use super::Side;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::symbol::Symbol;

mod tag {
    crate::string_field_impl!(Trade, "trade");
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    #[serde(rename = "type")]
    ty: tag::Trade,
    pub symbol: Symbol,
    pub event_id: u64,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_trade_deserialize() {
        let json = r#"{
            "type": "trade", 
            "symbol": "BTCUSD",
            "event_id": 42,
            "timestamp": 151231241,
            "price": "123.45",
            "quantity": "67.89",
            "side": "sell"
        }"#;

        let trade = serde_json::from_str::<Trade>(json).unwrap();

        assert!(matches!(trade.symbol, Symbol::BTCUSD));
        assert_eq!(trade.event_id, 42);
        assert_eq!(trade.timestamp.timestamp_millis(), 151231241);
        assert_eq!(trade.price, dec!(123.45));
        assert_eq!(trade.quantity, dec!(67.89));
        assert!(matches!(trade.side, Side::Sell));
    }
}
