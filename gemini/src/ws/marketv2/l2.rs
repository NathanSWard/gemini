use crate::symbol::Symbol;
use rust_decimal::Decimal;
use serde::Deserialize;

pub use super::auction::{Auction, AuctionData, AuctionIndicative, AuctionResult};
pub use super::trade::Trade;

#[derive(Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct ChangeData {
    price_level: Decimal,
    quantity: Decimal,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(from = "ChangeRepr")]
pub enum Change {
    Buy(ChangeData),
    Sell(ChangeData),
}

type ChangeRepr = (ChangeType, Decimal, Decimal);

impl From<ChangeRepr> for Change {
    fn from(repr: ChangeRepr) -> Self {
        let data = ChangeData {
            price_level: repr.1,
            quantity: repr.2,
        };
        match repr.0 {
            ChangeType::Buy => Change::Buy(data),
            ChangeType::Sell => Change::Sell(data),
        }
    }
}

mod tag {
    crate::string_field_impl!(L2Updates, "l2_updates");
}

#[derive(Deserialize, Debug, Clone)]
pub struct L2Data {
    #[serde(rename = "type")]
    ty: tag::L2Updates,
    pub symbol: Symbol,
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct L2Initial {
    #[serde(flatten)]
    pub data: L2Data,
    pub trades: Vec<Trade>,
    pub auction_events: Vec<Auction>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct L2Updates {
    #[serde(flatten)]
    pub data: L2Data,
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_l2update_deserialization() {
        let json = r#"{
            "type": "l2_updates",
            "symbol": "BTCUSD",
            "changes": [
                [
                    "sell",
                    "987.65",
                    "0.123"
                ],
                [
                    "buy",
                    "123.45",
                    "0.456"
                ]
            ]
        }"#;

        let l2 = serde_json::from_str::<L2Updates>(json).unwrap();

        assert!(matches!(l2.data.symbol, Symbol::BTCUSD));
        assert_eq!(2, l2.data.changes.len());
        assert!(matches!(l2.data.changes[0], Change::Sell(ChangeData {
                price_level,
                quantity,
            }) if price_level == dec!(987.65) && quantity == dec!(0.123)));
        assert!(matches!(l2.data.changes[1], Change::Buy(ChangeData {
                price_level,
                quantity,
            }) if price_level == dec!(123.45) && quantity == dec!(0.456)));
    }
}
