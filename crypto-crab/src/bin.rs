use crypto_crab::{algo::Algorithm, candle::Candle};
use crypto_crab_macros::*;

fn main() {}

#[derive(Algorithm)]
#[algo(symbol = "ETHUSD", resolution = "minute")]
pub struct MyAlgo;

impl Algorithm for MyAlgo {
    fn on(&mut self, data: &Self::Data) {
        data.get_ethusd();
    }
}
