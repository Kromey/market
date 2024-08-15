use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq)]
pub struct Buyer {
    bid_price: u32,
    strikes: u8,
}

impl Ord for Buyer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bid_price.cmp(&other.bid_price)
    }
}

impl PartialOrd for Buyer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Buyer {
    pub fn new(bid_price: u32) -> Self {
        Self {
            bid_price,
            strikes: 0,
        }
    }

    pub fn bid_price(&self) -> u32 {
        self.bid_price
    }
    
    pub fn strikes(&self) -> u8 {
        self.strikes
    }

    pub fn resolve_offer(&mut self, price: Option<u32>) {
        if price.is_some() {
            self.strikes = 0;
        } else {
            self.strikes += 1;
        }
    }
    
    pub fn close_day(&mut self, price: u32) {
        let price = if price == 0 {
            self.bid_price
        } else {
            price
        };

        if self.strikes > 0 {
            // I failed to buy today, raise my bid
            self.bid_price = max(self.bid_price + self.bid_price / 2, price + price / 2)
        } else if price < self.bid_price {
            // I paid too much! Reduce my bid toward the market value
            self.bid_price = lerp(price, self.bid_price, 0.5);
        }
    }
}

fn lerp(a: u32, b: u32, t: f32) -> u32 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u32
}

#[derive(Debug, PartialEq, Eq)]
pub struct Seller {
    ask_price: u32,
    strikes: u8,
}

impl Ord for Seller {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ask_price.cmp(&other.ask_price)
    }
}

impl PartialOrd for Seller {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Seller {
    pub fn new(ask_price: u32) -> Self {
        Self {
            ask_price,
            strikes: 0,
        }
    }

    pub fn ask_price(&self) -> u32 {
        self.ask_price
    }

    pub fn resolve_offer(&mut self, price: Option<u32>) {
        if price.is_some() {
            self.strikes = 0;
        } else {
            self.strikes += 1;
        }
    }
    
    pub fn close_day(&mut self, price: u32) {
        let price = if price == 0 {
            self.ask_price
        } else {
            price
        };

        if self.strikes > 0 {
            // I failed to sell today, reduce my ask
            self.ask_price = min(self.ask_price - self.ask_price / 2, price - price / 2)
        } else if price > self.ask_price {
            // I sold too low! Raise my price toward market value
            self.ask_price = lerp(price, self.ask_price, 0.5);
        }
    }
    
    pub fn strikes(&self) -> u8 {
        self.strikes
    }
}
