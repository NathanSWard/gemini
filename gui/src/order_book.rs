use crate::app::Message;
use iced::{Align, Color, Length};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Order {
    price: Decimal,
    quantity: Decimal,
}

#[derive(Debug, Default)]
pub struct OrderBook {
    asks: Vec<Order>,
    bids: Vec<Order>,
}

impl OrderBook {
    fn order_row<C: Into<Color> + Clone>(
        orders: &[Order],
        color: C,
    ) -> impl std::iter::Iterator<Item = iced::Element<'_, Message>> {
        orders.iter().map(move |order| {
            iced::Row::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .align_items(Align::Center)
                .push(
                    iced::Text::new(format!("{:?}:{:?}", order.price, order.quantity))
                        .color(color.clone()),
                )
                .into()
        })
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let asks = Self::order_row(&self.asks, Color::from_rgb8(40, 180, 99));
        let bids = Self::order_row(&self.bids, Color::from_rgb8(231, 76, 60));

        let mut column = iced::Column::new()
            .width(Length::Shrink)
            .height(Length::Shrink)
            .align_items(Align::Center)
            .padding(5)
            .spacing(10);

        for elem in asks.chain(bids) {
            column = column.push(elem);
        }

        column.into()
    }
}
