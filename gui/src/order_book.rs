use crate::app::Message;
use gemini::ws::market::{
    change::{Change, Side},
    Event, Update,
};
use iced::{Align, Color, Length};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Order {
    price: Decimal,
    quantity: Decimal,
}

impl std::cmp::PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price.eq(&other.price)
    }
}

impl std::cmp::Eq for Order {}

impl std::cmp::PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl std::cmp::Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

#[derive(Debug, Default)]
pub struct OrderBook {
    asks: Vec<Order>,
    bids: Vec<Order>,
}

impl OrderBook {
    fn order_row<'a, C: Into<Color> + Clone>(
        orders: impl Iterator<Item = &'a Order>,
        color: C,
    ) -> impl std::iter::Iterator<Item = iced::Element<'a, Message>> {
        orders.map(move |order| {
            iced::Text::new(format!(
                "Price: `{:?}` Quantity: `{:?}`",
                order.price, order.quantity
            ))
            .color(color.clone())
            .into()
        })
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let asks = Self::order_row(self.asks.iter().take(10), Color::from_rgb8(231, 76, 60));
        let bids = Self::order_row(self.bids.iter().take(10), Color::from_rgb8(40, 180, 99));

        let mut asks_column = iced::Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center);
        for ask in asks {
            asks_column = asks_column.push(ask);
        }

        let mut bids_column = iced::Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center);
        for bid in bids {
            bids_column = bids_column.push(bid);
        }

        iced::Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .push(asks_column)
            .push(bids_column)
            .into()
    }

    fn add_change(&mut self, change: Change) {
        let vec = match change.side {
            Side::Ask => &mut self.asks,
            Side::Bid => &mut self.bids,
        };

        vec.push(Order {
            price: change.price,
            quantity: change.remaining,
        });
    }

    fn update_change(&mut self, change: Change) {
        let orders = match change.side {
            Side::Ask => &mut self.asks,
            Side::Bid => &mut self.bids,
        };

        let price = change.price;
        match orders
            .iter_mut()
            .enumerate()
            .find(|(_, o)| o.price == price)
        {
            Some((idx, order)) => {
                if change.remaining.is_zero() {
                    orders.remove(idx);
                } else {
                    order.quantity = change.remaining;
                }
            }
            None => orders.push(Order {
                price: change.price,
                quantity: change.remaining,
            }),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Init(update) => {
                for event in update.events {
                    if let Event::Change(change) = event {
                        self.add_change(change);
                    }
                }
                println!("bids: {}, asks: {}", self.bids.len(), self.asks.len());
            }
            Message::Change(update) => {
                for change in update.events {
                    self.update_change(change);
                }
            }
            _ => {}
        }

        // TODO: don't always sort.
        // Use a better data structure
        self.asks.sort_unstable();
        self.bids.sort_unstable_by(|o1, o2| o2.cmp(o1));
    }
}
