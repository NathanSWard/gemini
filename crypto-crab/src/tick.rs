use rust_decimal::Decimal;

pub enum Side {
    Buy,
    Sell,
}

pub struct Tick {
    time: crate::Date,
    price: Decimal,
    side: Side,
    quantity: Decimal,
}
