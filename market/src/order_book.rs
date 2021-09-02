use rust_decimal::Decimal;
use std::{borrow::Borrow, collections::BTreeSet};

macro_rules! decimal_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $name(pub Decimal);

        impl AsRef<Decimal> for $name {
            fn as_ref(&self) -> &Decimal {
                &self.0
            }
        }

        impl AsMut<Decimal> for $name {
            fn as_mut(&mut self) -> &mut Decimal {
                &mut self.0
            }
        }

        impl From<Decimal> for $name {
            fn from(d: Decimal) -> $name {
                $name(d)
            }
        }

        impl From<$name> for Decimal {
            fn from(x: $name) -> Decimal {
                x.0
            }
        }
    };
}

decimal_wrapper!(Quantity);
decimal_wrapper!(Price);

impl std::cmp::PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl std::cmp::Ord for Price {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl Borrow<Price> for OrderData {
    fn borrow(&self) -> &Price {
        &self.price
    }
}

#[derive(Debug, Clone)]
pub struct OrderData {
    pub price: Price,
    pub quantity: Quantity,
}

impl OrderData {
    pub fn new(price: impl Into<Decimal>, quantity: impl Into<Decimal>) -> Self {
        Self {
            price: Price(price.into()),
            quantity: Quantity(quantity.into()),
        }
    }
}

impl std::cmp::PartialEq for OrderData {
    fn eq(&self, other: &Self) -> bool {
        self.price.eq(&other.price)
    }
}

impl std::cmp::Eq for OrderData {}

impl std::cmp::PartialOrd for OrderData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl std::cmp::Ord for OrderData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

#[derive(Debug, Clone)]
pub enum Order<T> {
    Bid(T),
    Ask(T),
}

#[derive(Default, Clone, Debug)]
pub struct OrderBook {
    asks: BTreeSet<OrderData>,
    bids: BTreeSet<OrderData>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter_asks(
        &self,
    ) -> impl Iterator<Item = &OrderData> + DoubleEndedIterator + ExactSizeIterator {
        self.asks.iter().rev()
    }

    pub fn iter_bids(
        &self,
    ) -> impl Iterator<Item = &OrderData> + DoubleEndedIterator + ExactSizeIterator {
        self.bids.iter()
    }

    pub fn asks_len(&self) -> usize {
        self.asks.len()
    }

    pub fn bids_len(&self) -> usize {
        self.bids.len()
    }

    pub fn top(&self, order_type: Order<()>) -> Option<&OrderData> {
        match order_type {
            Order::Bid(()) => self.iter_bids().next(),
            Order::Ask(()) => self.iter_asks().next(),
        }
    }

    fn update_impl(set: &mut BTreeSet<OrderData>, order: OrderData) {
        if order.quantity.0.is_zero() {
            set.remove(&order.price);
        } else {
            set.replace(order);
        }
    }

    pub fn update_asks(&mut self, order: OrderData) {
        Self::update_impl(&mut self.asks, order);
    }

    pub fn update_bids(&mut self, order: OrderData) {
        Self::update_impl(&mut self.bids, order);
    }

    pub fn update(&mut self, order: Order<OrderData>) {
        match order {
            Order::Ask(data) => self.update_asks(data),
            Order::Bid(data) => self.update_bids(data),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_order_book() {
        let mut book = OrderBook::new();

        book.update(Order::Ask(OrderData::new(dec!(1.0), dec!(1.0))));
        assert_eq!(1, book.asks_len());

        book.update(Order::Ask(OrderData::new(dec!(2.0), dec!(2.0))));
        assert_eq!(2, book.asks_len());

        book.update(Order::Ask(OrderData::new(dec!(2.0), dec!(1.0))));
        assert_eq!(2, book.asks_len());

        book.update(Order::Ask(OrderData::new(dec!(2.0), dec!(0))));
        assert_eq!(1, book.asks_len());
    }
}
