use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Reason {
    Place,
    Trade,
    Cancel,
    Initial,
}

pub mod tag {
    crate::string_field_impl!(Change, "change");
}

#[derive(Deserialize, Clone, Debug)]
pub struct Change {
    #[serde(rename = "type")]
    pub ty: tag::Change,

    // The price of this order book entry.
    pub price: Decimal,

    pub side: Side,

    // Either place, trade, cancel, or initial, to indicate why the change has occurred.
    // initial is for the initial response message, which will show the entire existing state of the order book.
    pub reason: Reason,

    // The quantity remaining at that price level after this change occurred.
    // May be zero if all orders at this price level have been filled or canceled.
    pub remaining: Decimal,

    // The quantity changed. May be negative, if an order is filled or canceled.
    // For initial messages, delta will equal remaining.
    pub delta: Decimal,
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use rust_decimal_macros::dec;

    pub const CHANGE_TEST_STRINGS: &[&str] = &[CHANGE];

    const CHANGE: &str = r#"{
        "type": "change",
        "side": "bid",
        "price": "3626.73",
        "remaining": "1.6",
        "delta": "0.8",
        "reason": "place"
    }"#;

    #[test]
    fn test_change_deserialize() {
        let change = serde_json::from_str::<Change>(CHANGE).unwrap();

        assert!(matches!(change.side, Side::Bid));
        assert_eq!(change.price, dec!(3626.73));
        assert_eq!(change.remaining, dec!(1.6));
        assert_eq!(change.delta, dec!(0.8));
        assert_eq!(change.reason, Reason::Place);
    }
}
