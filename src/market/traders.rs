use std::cmp::{max, min};

const MIN_UNRESOLVED_OFFER_CHANGE: u32 = 5;

#[derive(Debug, PartialEq, Eq)]
pub struct Buyer {
    max_price: u32,
    bid_price: u32,
}

impl Buyer {
    pub fn new(max_price: u32) -> Self {
        Self {
            max_price,
            bid_price: max_price / 2,
        }
    }

    pub fn bid_price(&self) -> u32 {
        self.bid_price
    }

    pub fn resolve_offer(&mut self, price: Option<u32>) {
        if let Some(price) = price {
            self.bid_price = ((self.bid_price as f32 + price as f32) / 2.0).round() as u32;
        } else {
            let adj = min(self.bid_price / 3, MIN_UNRESOLVED_OFFER_CHANGE);
            self.bid_price = self.bid_price.saturating_add(adj);
        }

        self.bid_price = min(self.bid_price, self.max_price);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Seller {
    min_price: u32,
    ask_price: u32,
}

impl Seller {
    pub fn new(min_price: u32) -> Self {
        Self {
            min_price,
            ask_price: min_price.saturating_add(min_price / 2),
        }
    }

    pub fn ask_price(&self) -> u32 {
        self.ask_price
    }

    pub fn resolve_offer(&mut self, price: Option<u32>) {
        if let Some(price) = price {
            self.ask_price = ((self.ask_price as f32 + price as f32) / 2.0).round() as u32;
        } else {
            let adj = min(self.ask_price / 3, MIN_UNRESOLVED_OFFER_CHANGE);
            self.ask_price = self.ask_price.saturating_sub(adj);
        }

        self.ask_price = max(self.ask_price, self.min_price);
    }
}
