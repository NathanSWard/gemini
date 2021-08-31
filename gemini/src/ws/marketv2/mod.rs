pub mod auction;
pub mod candle;
pub mod l2;
pub mod trade;

use super::heartbeat::Heartbeat;
use crate::symbol::Symbol;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}

mod tag {
    crate::string_field_impl!(Subscribe, "subscribe");
    crate::string_field_impl!(Unsubscribe, "unsubscribe");
}

#[derive(Serialize, Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum SubscriptionType {
    #[serde(rename = "l2")]
    L2,
    #[serde(rename = "candles_1m")]
    Candles1m,
    #[serde(rename = "candles_5m")]
    Candles5m,
    #[serde(rename = "candles_15m")]
    Candles15m,
    #[serde(rename = "candles_30m")]
    Candles30m,
    #[serde(rename = "candles_1h")]
    Candles1h,
    #[serde(rename = "candles_6h")]
    Candles6h,
    #[serde(rename = "candles_1d")]
    Candles1d,
}

#[derive(Serialize, Clone, Debug, TypedBuilder)]
pub struct Subscription {
    name: SubscriptionType,
    #[builder(setter(into))]
    symbols: Vec<Symbol>,
}

#[derive(Serialize, Clone, Debug, TypedBuilder)]
pub struct Subscribe {
    #[builder(default, setter(skip))]
    #[serde(rename = "type")]
    ty: tag::Subscribe,
    #[builder(setter(into))]
    subscriptions: Vec<Subscription>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Unsubscribe {
    #[serde(rename = "type")]
    ty: tag::Unsubscribe,
    subscriptions: Vec<Subscription>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct HeartbeatData {
    #[serde(with = "ts_milliseconds")]
    timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Response<D> {
    Heartbeat(Heartbeat<HeartbeatData>),
    Data(D),
}

#[cfg(test)]
mod test {
    #[test]
    fn test_subscribe_serialize() {}
}
