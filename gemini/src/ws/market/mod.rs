pub mod auction;
pub mod change;
pub mod trade;

use super::heartbeat::Heartbeat;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::symbol::Symbol;

use super::client::WssRequest;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Event {
    Trade(trade::Trade),
    Change(change::Change),
    BlockTrade(trade::BlockTrade),
    // TODO: Maybe just expand the Auction types in place
    // to optimize tagging?
    Auction(auction::Auction),
}

#[derive(Deserialize, Clone, Debug)]
pub struct Update {
    socket_sequence: u32,
    #[serde(rename = "eventId")]
    event_id: u32,
    // TODO: this doesn't exist??
    // #[serde(rename = "timestampms", with = "ts_milliseconds")]
    // timestamp: DateTime<Utc>,
    events: Vec<Event>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Response {
    // TODO
    //Heartbeat(Heartbeat),
    Update(Update),
}

#[derive(TypedBuilder, Clone, Debug)]
pub struct MarketWssRequest {
    symbol: Symbol,
    #[builder(default = false)]
    heartbeat: bool,
    #[builder(default = false)]
    top_of_book: bool,
    #[builder(default = false)]
    bids: bool,
    #[builder(default = false)]
    offers: bool,
    #[builder(default = false)]
    trades: bool,
    #[builder(default = false)]
    auctions: bool,
}

impl WssRequest for MarketWssRequest {
    type Response = Response;

    fn url(self, mut url: reqwest::Url) -> reqwest::Url {
        url.path_segments_mut()
            .unwrap()
            .extend(["v1", "marketdata", self.symbol.as_ref()]);

        {
            let mut query = url.query_pairs_mut();
            let ser = serde_urlencoded::Serializer::new(&mut query);
            (
                ("heartbeat", self.heartbeat),
                ("top_of_book", self.top_of_book),
                ("bids", self.bids),
                ("offers", self.offers),
                ("trades", self.trades),
                ("auctions", self.auctions),
            )
                .serialize(ser)
                .unwrap();
        }

        url
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_event_impl {
        ($strings:path, $match:pat_param) => {
            for s in $strings {
                assert!(matches!(serde_json::from_str::<Event>(s).unwrap(), $match));
            }
        };
    }

    #[test]
    fn test_event_deserialize() {
        test_event_impl!(
            crate::ws::market::change::test::CHANGE_TEST_STRINGS,
            Event::Change(_)
        );

        test_event_impl!(
            crate::ws::market::auction::test::AUCTION_TEST_STRINGS,
            Event::Auction(_)
        );

        test_event_impl!(
            crate::ws::market::trade::test::TRADE_TEST_STRINGS,
            Event::Trade(_)
        );

        test_event_impl!(
            crate::ws::market::trade::test::BLOCK_TRADE_TEST_STRINGS,
            Event::BlockTrade(_)
        );
    }
}
