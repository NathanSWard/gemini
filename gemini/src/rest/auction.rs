use crate::{rest::HttpRequest, symbol::Symbol};
use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use typed_builder::TypedBuilder;

#[derive(Deserialize, Debug, Clone)]
pub struct Auction {
    // If the auction is not currently open, show the time at which the next auction opens.
    // Not present if the auction has already opened.
    #[serde(with = "ts_milliseconds_option")]
    pub closed_until_ms: Option<DateTime<Utc>>,

    // After an auction opens, the unique event ID for last specific auction event.
    // Changes when an auction event occurs: the auction opens, an indicative price is published, and the auction itself runs.
    // Not present before the auction opens.
    pub last_auction_edi: Option<u32>,

    // If available, show the auction price from the last successful auction for this trading pair.
    // Not present after current auction begins publishing indicative prices.
    pub last_auction_price: Option<Decimal>,

    // If available, show the auction quantity from the last successful auction for this trading pair.
    // Not present after current auction begins publishing indicative prices.
    pub last_auction_quantity: Option<Decimal>,

    // If available, show the highest bid price from the continuous trading order book at the time of the last successful auction for this trading pair.
    // Not present after current auction begins publishing indicative prices.
    pub last_highest_bid_price: Option<Decimal>,

    // If available, show the lowest ask price from the continuous trading order book at the time of the last successful auction for this trading pair.
    // Not present after current auction begins publishing indicative prices.
    pub last_lowest_ask_price: Option<Decimal>,

    // If available, show the collar price at the time of the last successful auction for this trading pair.
    // Not present after current auction begins publishing indicative prices.
    pub last_collar_price: Option<Decimal>,

    // The most recently published indicative price for the auction.
    // Not present before the current auction begins publishing indicatives.
    pub most_recent_indicative_price: Option<Decimal>,

    // The most recently published indicative quantity for the auction.
    // Not present before the current auction begins publishing indicatives.
    pub most_recent_indicative_quantity: Option<Decimal>,

    // The most recent highest bid at the time of the indicative price for the auction.
    // Not present before the current auction begins publishing indicatives.
    pub most_recent_highest_bid_price: Option<Decimal>,

    // The most recent lowest ask at the time of the indicative price for the auction.
    // Not present before the current auction begins publishing indicatives.
    pub most_recent_lowest_ask_price: Option<Decimal>,

    // The most recent collar price at the time of the indicative price for the auction.
    // Not present before the current auction begins publishing indicatives.
    pub most_recent_collar_price: Option<Decimal>,

    // Timestamp of the next event in this auction, either the publication of an indicative price/quantity or the auction itself.
    #[serde(with = "ts_milliseconds_option")]
    pub next_update_ms: Option<DateTime<Utc>>,

    // Timestamp of when the next auction will run.
    #[serde(with = "ts_milliseconds")]
    pub next_auction_ms: DateTime<Utc>,
}

#[derive(TypedBuilder)]
pub struct AuctionHttpRequest {
    symbol: Symbol,
}

impl HttpRequest for AuctionHttpRequest {
    type Response = Auction;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn url(self, mut url: reqwest::Url) -> reqwest::Url {
        url.path_segments_mut()
            .unwrap()
            .extend(["v1", self.symbol.as_ref()]);

        url
    }
}
