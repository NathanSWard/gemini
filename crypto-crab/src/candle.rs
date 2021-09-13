use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Candle {
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
}
