use crate::decimal::*;
use async_tungstenite::tungstenite;
use chrono::{Date, DateTime, Duration, Local, TimeZone, Utc};
use futures_util::{SinkExt, StreamExt};
use gemini::{
    symbol::Symbol,
    ws::{
        self,
        marketv2::{l2::*, trade::Trade},
    },
};
use iced::{
    executor, Align, Application, Clipboard, Column, Command, Container, Element, Font, Length,
    Settings, Subscription,
};
use itertools::Itertools;
use market::stats::Stats;
use plotters::{prelude::*, style::IntoFont};
use plotters_iced::{Chart, ChartWidget};
use reqwest::header::InvalidHeaderName;
use rust_decimal::{prelude::ToPrimitive, Decimal};
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
                        let (mut stream, _) = gemini::ws::client::connect_wss(
                            reqwest::Url::parse("wss://api.gemini.com/v2/marketdata").unwrap(),
                        )
                        .await
                        .unwrap();

                        let req = ws::marketv2::Subscribe::builder()
                            .subscriptions(vec![ws::marketv2::Subscription::builder()
                                .name(ws::marketv2::SubscriptionType::L2)
                                .symbols(vec![Symbol::BTCUSD])
                                .build()])
                            .build();
                        stream
                            .send(tungstenite::Message::Text(
                                serde_json::to_string(&req).unwrap(),
                            ))
                            .await
                            .unwrap();

                        let initial = match stream.next().await {
                            Some(Ok(tungstenite::Message::Text(text))) => {
                                serde_json::from_str::<gemini::ws::marketv2::l2::L2Initial>(&text)
                                    .unwrap()
                            }
                            Some(e) => panic!("{:?}", e),
                            None => panic!("AHH"),
                        };

                        Some((Message::Init(initial), State::Polling(stream)))
                    }
                    State::Polling(mut stream) => match stream.next().await {
                        Some(Ok(tungstenite::Message::Text(text))) => {
                            if let Ok(trade) = serde_json::from_str(&text) {
                                Some((Message::Trade(trade), State::Polling(stream)))
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
    Init(ws::marketv2::l2::L2Initial),
    Trade(ws::marketv2::trade::Trade),
    Other(String),
}

pub struct App {
    ob: StatsChart,
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

#[derive(Debug)]
pub struct MeanDate {
    mean: Decimal,
    date: DateTime<Utc>,
    stddev: Decimal,
}

#[derive(Default, Debug)]
pub struct StatsChart {
    price: Decimal,
    curr: Stats<Decimal>,
    devs: Vec<Decimal>,
    data: Vec<MeanDate>,
}

impl StatsChart {
    pub fn view(&mut self) -> iced::Element<'_, Message> {
        ChartWidget::new(self)
            .width(Length::Units(800))
            .height(Length::Units(800))
            .into()
    }

    pub fn update(&mut self, message: Message) {
        let mut add_trade = |trade: Trade| {
            self.price = trade.price;
            self.curr.add(trade.price);
            self.data.push(MeanDate {
                mean: self.curr.mean(),
                date: trade.timestamp,
                stddev: self.curr.stddev(),
            });
        };

        match message {
            Message::Init(mut init) => {
                for trade in init
                    .trades
                    .drain(..)
                    .sorted_by(|a, b| a.timestamp.cmp(&b.timestamp))
                {
                    add_trade(trade);
                }
            }
            Message::Trade(trade) => {
                add_trade(trade);
            }
            _ => {}
        }
    }
}

impl Chart<Message> for StatsChart {
    fn build_chart<DB: DrawingBackend>(&self, mut builder: ChartBuilder<'_, '_, DB>) {
        if self.data.len() <= 1 {
            return;
        }

        let (from_date, to_date) = match self.data.iter().map(|data| data.date).minmax() {
            itertools::MinMaxResult::MinMax(min, max) => (min, max),
            _ => panic!("ahh"),
        };

        let from_price = self
            .data
            .iter()
            .map(|data| data.mean - data.stddev)
            .min()
            .unwrap();
        let to_price = self
            .data
            .iter()
            .map(|data| data.mean + data.stddev)
            .max()
            .unwrap();

        let std = self.curr.stddev() * dec!(2);

        let mut chart = builder
            .caption(
                format!("Current Price: {}", self.price.round_dp(2)),
                ("sans-serif", 50.0).into_font(),
            )
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                from_date..to_date,
                DecimalRange {
                    start: from_price.floor() - std,
                    end: to_price.ceil() + std,
                },
            )
            .unwrap();

        chart
            .configure_mesh()
            .light_line_style(&WHITE)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                self.data.iter().map(|s| (s.date, s.mean)),
                &BLUE,
            ))
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                self.data
                    .iter()
                    .map(|data| (data.date, data.mean + data.stddev)),
                &GREEN,
            ))
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                self.data
                    .iter()
                    .map(|data| (data.date, data.mean - data.stddev)),
                &RED,
            ))
            .unwrap();
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
