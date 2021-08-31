use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MakerSide {
    Bid,
    Ask,
    Auction,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TradeData {
    // The trade id.
    pub tid: u64,

    // The price this trade executed at.
    pub price: Decimal,

    // The amount traded.
    pub amount: Decimal,
}

crate::string_field_impl!(TradeTag, "trade");

#[derive(Deserialize, Clone, Debug)]
pub struct Trade {
    #[serde(rename = "type")]
    pub ty: TradeTag,

    #[serde(flatten)]
    pub data: TradeData,

    // The side of the book the maker of the trade placed their order on, of if the trade occurred in an auction.
    #[serde(rename = "makerSide")]
    pub maker_side: MakerSide,
}

crate::string_field_impl!(BlockTradeTag, "block_trade");

#[derive(Deserialize, Clone, Debug)]
pub struct BlockTrade {
    #[serde(rename = "type")]
    pub ty: BlockTradeTag,

    #[serde(flatten)]
    pub data: TradeData,
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use rust_decimal_macros::dec;

    pub const TRADE_TEST_STRINGS: &[&str] = &[TRADE];
    pub const BLOCK_TRADE_TEST_STRINGS: &[&str] = &[BLOCK_TRADE];

    const TRADE: &str = r#"{
        "type": "trade",
        "tid": 5375547515,
        "price": "3632.54",
        "amount": "0.1362819142",
        "makerSide": "ask"
      }"#;

    const BLOCK_TRADE: &str = r#"{
        "type":"block_trade",
        "tid":1111597035,
        "price":"10100.00",
        "amount":"1000"
     }"#;

    #[test]
    fn test_trade_deserialize() {
        let trade = serde_json::from_str::<Trade>(TRADE).unwrap();

        assert_eq!(trade.data.tid, 5375547515);
        assert_eq!(trade.data.price, dec!(3632.54));
        assert_eq!(trade.data.amount, dec!(0.1362819142));
        assert!(matches!(trade.maker_side, MakerSide::Ask));
    }

    #[test]
    fn test_block_trade_deserialize() {
        let trade = serde_json::from_str::<BlockTrade>(BLOCK_TRADE).unwrap();

        assert_eq!(trade.data.tid, 1111597035);
        assert_eq!(trade.data.price, dec!(10100.00));
        assert_eq!(trade.data.amount, dec!(1000));
    }
}
