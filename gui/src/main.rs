use async_tungstenite::tungstenite::Message;
use chrono::{DateTime, Local, TimeZone};
use futures_util::{SinkExt, StreamExt};
use gemini::{
    rest::{
        candles::{CandleHttpRequest, Candles, TimeRange},
        client::{Client, Error},
    },
    symbol::Symbol,
};
use itertools::{Itertools, MinMaxResult::MinMax};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::convert::TryFrom;

fn dec_to_f32(dec: Decimal) -> f32 {
    f32::try_from(dec).unwrap()
}

fn to_local<Z: TimeZone>(date: DateTime<Z>) -> DateTime<Local> {
    date.with_timezone(&Local)
}

/*
fn candle_stick_graph() -> egui::plot::Plot {
    use egui::plot::{Line, Plot, Value, Values};
    let n = 128;
    let line = Line::new(Values::from_values_iter((0..=n).map(|i| {
        use std::f64::consts::TAU;
        let x = egui::remap(i as f64, 0.0..=(n as f64), -TAU..=TAU);
        Value::new(x, x.sin())
    })));
    Plot::new("example_plot")
        .line(line)
        .height(32.0)
        .data_aspect(1.0)
}
*/

fn main() {
    let app = gui::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

/*
#[tokio::main]
async fn main() {
    let client = Client::new(reqwest::Url::parse("https://api.gemini.com").unwrap());

    let req = CandleHttpRequest::builder()
        .symbol(Symbol::BTCUSD)
        .time_range(TimeRange::Day1)
        .build();

    let candles = match client.request(req).await {
        Ok(candles) => candles,
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            return;
        }
    };

    println!(
        "START: {:?}\nEND: {:?}",
        to_local(candles.candles.last().unwrap().time),
        to_local(candles.candles.first().unwrap().time)
    );

    let (low_price, high_price) = match candles.candles.iter().map(|candle| candle.close).minmax() {
        MinMax(low, high) => (low, high),
        _ => panic!("No min/max :("),
    };

    println!("LOW: {}, HIGH: {}", low_price, high_price);

    let start_date = Local.ymd(2021, 8, 28).and_hms(0, 0, 0);
}
*/
