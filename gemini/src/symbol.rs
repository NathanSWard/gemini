use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(
    Deserialize,
    Serialize,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    AsRefStr,
    Display,
    EnumString,
    IntoStaticStr,
)]
#[serde(try_from = "&str", into = "&str")]
pub enum Symbol {
    #[strum(ascii_case_insensitive)]
    BTCUSD,
    #[strum(ascii_case_insensitive)]
    ETHUSD,
    #[serde(other)]
    Unknown,
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}
