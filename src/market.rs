use rand::prelude::*;
use rand_xoshiro::Xoshiro256StarStar;

use self::{traders::{Buyer, Seller}, history::History};
pub use self::history::Statistics;

mod traders;
mod history;

#[derive(Debug, PartialEq, Eq)]
pub struct Market {
    buyers: Vec<Buyer>,
    sellers: Vec<Seller>,
    history: History<25>,
    rng: Xoshiro256StarStar,
}

impl Market {
    pub fn new(price: u32, trade_volume: u32) -> Self {
        let buyer_limit = (price as f32 * 1.25).round() as u32;
        let seller_limit = (price as f32 * 0.75).round() as u32;

        let mut rng = Xoshiro256StarStar::from_entropy();

        let buyer_volume = trade_volume;
        let seller_volume = trade_volume + (trade_volume / 2);

        Self {
            buyers: (0..buyer_volume)
                .map(|_| Buyer::new(rng.gen_range(price..=buyer_limit)))
                .collect(),
            sellers: (0..seller_volume)
                .map(|_| Seller::new(rng.gen_range(seller_limit..=price)))
                .collect(),
            history: History::new(),
            rng: Xoshiro256StarStar::from_entropy(),
        }
    }

    pub fn get_latest_stats(&self) -> Statistics {
        self.history.get_latest()
    }

    pub fn get_total_stats(&self) -> Statistics {
        self.history.get_totals()
    }

    pub fn do_trades(&mut self) {
        self.buyers.retain(|buyer| buyer.strikes() < 2);
        self.sellers.retain(|seller| seller.strikes() < 2);
        println!("\tBuyers: {}; Sellers: {}", self.buyers.len(), self.sellers.len());

        let mut buyers: Vec<_> = self.buyers.iter_mut().collect();
        let mut sellers: Vec<_> = self.sellers.iter_mut().collect();

        let mut volume = 0;
        let mut value = 0;

        // buyers.shuffle(&mut self.rng);
        buyers.sort();
        buyers.reverse();

        sellers.sort();

        for (buyer, seller) in buyers.into_iter().zip(sellers.into_iter()) {
            let ask = seller.ask_price() as f32;
            let bid = buyer.bid_price() as f32;

            if bid > ask {
                let price = ((bid + ask) / 2.0).round() as u32;
                seller.resolve_offer(Some(price));
                buyer.resolve_offer(Some(price));

                volume += 1;
                value += price;
            } else {
                buyer.resolve_offer(None);
                seller.resolve_offer(None);
            }
        }

        self.history.insert(volume, value);
        let price = self.history.get_totals().price();
        for buyer in self.buyers.iter_mut() {
            buyer.close_day(price);
        }
        for seller in self.sellers.iter_mut() {
            seller.close_day(price);
        }
    }

    pub fn buy(&mut self, price: u32, qty: u32) -> u32 {
        let mut sellers: Vec<_> = self.sellers.iter_mut().collect();

        let mut volume = 0;

        sellers.shuffle(&mut self.rng);

        for seller in sellers.iter_mut() {
            if price >= seller.ask_price() {
                seller.resolve_offer(Some(price));
                seller.close_day(price);
                volume += 1;

                if volume == qty {
                    break;
                }
            }
        }

        volume
    }

    pub fn sell(&mut self, price: u32, qty: u32) -> u32 {
        let mut buyers: Vec<_> = self.buyers.iter_mut().collect();

        let mut volume = 0;

        buyers.shuffle(&mut self.rng);

        for buyer in buyers.iter_mut() {
            if price <= buyer.bid_price() {
                buyer.resolve_offer(Some(price));
                buyer.close_day(price);
                volume += 1;

                if volume == qty {
                    break;
                }
            }
        }

        volume
    }
}

impl Default for Market {
    fn default() -> Self {
        Self::new(30, 20)
    }
}
