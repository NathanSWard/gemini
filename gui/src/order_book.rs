use crate::app::Message;
use gemini::ws::market::{
    change::{Change, Side},
    Event,
};
use iced::widget::container::StyleSheet;
use iced::{Align, Background, Color, Length};
use market::order_book::{OrderBook, OrderData};

#[derive(Default, Debug, Clone)]
pub struct OrderBookChart {
    book: OrderBook,
}

impl OrderBookChart {
    fn order_row<'a, C: Into<Color> + Clone>(
        orders: impl Iterator<Item = &'a OrderData>,
        color: C,
    ) -> impl std::iter::Iterator<Item = iced::Element<'a, Message>> {
        orders.map(move |order| {
            iced::Text::new(format!(
                "Price: `{}` Quantity: `{}`",
                order.price.0, order.quantity.0
            ))
            .color(color.clone())
            .into()
        })
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        fn make_column<'a>(
            data: impl Iterator<Item = &'a OrderData>,
            color: Color,
        ) -> iced::Column<'a, Message> {
            let make_container = |text, color, align_x| {
                iced::Container::new(iced::Text::new(text).color(color))
                    .align_x(align_x)
                    .align_y(Align::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
            };

            let mut column = iced::Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Align::Center);

            for order_data in data {
                let row = iced::Row::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_items(Align::Center)
                    .push(make_container(
                        format!("{}", order_data.price.0),
                        color,
                        Align::Start,
                    ))
                    .push(make_container(
                        format!("{}", order_data.quantity.0),
                        color,
                        Align::End,
                    ));
                column = column.push(row);
            }

            column
        }

        let asks_column = make_column(
            self.book.iter_asks().take(10),
            Color::from_rgb8(231, 76, 60),
        );

        let bids_column = make_column(
            self.book.iter_bids().take(10),
            Color::from_rgb8(40, 180, 99),
        );

        iced::Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .push(asks_column)
            .push(bids_column)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        let mut update_order_book = |change: Change| {
            let data = OrderData::new(change.price, change.remaining);
            match change.side {
                Side::Ask => self.book.update_asks(data),
                Side::Bid => self.book.update_bids(data),
            }
        };

        /*
        match message {
            Message::Init(update) => {
                for event in update.events {
                    if let Event::Change(change) = event {
                        update_order_book(change);
                    }
                }
            }
            Message::Change(update) => {
                for change in update.events {
                    update_order_book(change);
                }
            }
            _ => {}
        }
        */
    }
}
