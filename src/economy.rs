use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

use crate::{Market, Statistics};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Economy<G>
where
    G: Hash + Eq,
{
    markets: HashMap<G, Market>,
}

impl<G: Hash + Eq> Economy<G> {
    pub fn new() -> Self {
        Self {
            markets: HashMap::new(),
        }
    }

    pub fn add_good(&mut self, good: G, price: u32, volume: u32) -> bool {
        if let Entry::Vacant(e) = self.markets.entry(good) {
            e.insert(Market::new(price, volume));
            true
        } else {
            false
        }
    }

    pub fn get_latest_stats(&self, good: &G) -> Option<Statistics> {
        Some(self.markets.get(good)?.get_latest_stats())
    }

    pub fn get_total_stats(&self, good: &G) -> Option<Statistics> {
        Some(self.markets.get(good)?.get_total_stats())
    }

    pub fn do_trades(&mut self) {
        self.markets.iter_mut().for_each(|(_, market)| {
            market.do_trades();
        });
    }

    pub fn bulk_buy(&mut self, good: &G, price: u32, qty: u32) -> Option<u32> {
        Some(self.markets.get_mut(good)?.bulk_buy(price, qty))
    }

    pub fn bulk_sell(&mut self, good: &G, price: u32, qty: u32) -> Option<u32> {
        Some(self.markets.get_mut(good)?.bulk_sell(price, qty))
    }
}
