use crate::decimal::DecimalRange;
use async_tungstenite::tungstenite;
use chrono::{Date, DateTime, Duration, Local, TimeZone, Utc};
use futures_util::{SinkExt, StreamExt};
use gemini::{
    symbol::Symbol,
    ws::{
        self,
        market::{change::Change, Event, Update},
    },
};
use iced::{
    executor, Align, Application, Clipboard, Column, Command, Container, Element, Font, Length,
    Settings, Subscription,
};
use plotters::{prelude::*, style::IntoFont};
use plotters_iced::{Chart, ChartWidget};
use reqwest::header::InvalidHeaderName;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;
use std::hash::Hasher;
use std::sync::{Arc, Mutex};

struct PollOB;

pub enum State<S: futures::Stream> {
    Start,
    Polling(S),
    End,
}

impl<H: Hasher, I> iced_native::subscription::Recipe<H, I> for PollOB {
    type Output = Message;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: iced_futures::BoxStream<I>,
    ) -> iced_futures::BoxStream<Self::Output> {
        Box::pin(futures::stream::unfold(
            State::<gemini::ws::client::WssStream>::Start,
            |state| async move {
                match state {
                    State::Start => {
                        let (mut stream, _) = gemini::ws::client::connect_wss_with_request(
                            reqwest::Url::parse("wss://api.gemini.com").unwrap(),
                            ws::market::MarketWssRequest::builder()
                                .bids(true)
                                .offers(true)
                                .symbol(Symbol::ETHUSD)
                                .build(),
                        )
                        .await
                        .unwrap();

                        let update = match stream.next().await {
                            Some(Ok(tungstenite::Message::Text(text))) => {
                                serde_json::from_str::<gemini::ws::market::Update>(&text).unwrap()
                            }
                            Some(e) => panic!("{:?}", e),
                            None => panic!("AHH"),
                        };

                        Some((Message::Init(update), State::Polling(stream)))
                    }
                    State::Polling(mut stream) => match stream.next().await {
                        Some(Ok(tungstenite::Message::Text(text))) => {
                            if let Ok(change) = serde_json::from_str(&text) {
                                Some((Message::Change(change), State::Polling(stream)))
                            } else {
                                Some((Message::Other(text), State::Polling(stream)))
                            }
                        }
                        Some(e) => panic!("{:?}", e),
                        None => panic!("ahh"),
                    },
                    State::End => None,
                }
            },
        ))
    }
}

fn parse_time(t: &str) -> Date<Local> {
    Local
        .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
        .unwrap()
        .date()
}

#[derive(Debug)]
pub enum Message {
    Waiting,
    Init(Update),
    Change(Update<Change>),
    Other(String),
}

pub struct App {
    ob: crate::order_book::OrderBookChart,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                ob: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Gemini GUI")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
        self.ob.update(message);
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Container::new(self.ob.view())
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::from_recipe(PollOB)
    }
}

/*
struct CandleChart;

impl CandleChart {
    fn view(&mut self) -> iced::Element<'_, Message> {
        ChartWidget::new(self)
            .width(Length::Units(800))
            .height(Length::Units(800))
            .into()
    }
}

impl Chart<Message> for CandleChart {
    fn build_chart<DB: plotters_iced::DrawingBackend>(
        &self,
        mut builder: plotters_iced::ChartBuilder<'_, '_, DB>,
    ) {

        let mut chart = builder
            .caption("Candle Chart", ("sans-serif", 50.0).into_font())
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(from_date..to_date, 110f32..135f32)
            .unwrap();

        chart
            .configure_mesh()
            .light_line_style(&WHITE)
            .draw()
            .unwrap();

        chart
            .draw_series(candles.iter().map(|candle| {
                crate::candlestick::CandleStick::new(
                    crate::candlestick::Candle {
                        time: parse_time(candle.0),
                        open: candle.1,
                        high: candle.2,
                        low: candle.3,
                        close: candle.4,
                    },
                    GREEN,
                    RED,
                    2,
                    15,
                )
            }))
            .unwrap();
    }
}
*/
