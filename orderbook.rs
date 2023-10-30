#![allow(dead_code)]
use std::collections::HashMap;
use rust_decimal::prelude::*;
#[derive(Debug)]

pub enum BidOrAsk {
    Bid,
    Ask
}
#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }
    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::Bid => self.ask_limits(),
            BidOrAsk::Ask => self.bid_limits()
        };
        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }

    }
    
    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits: Vec<&mut Limit> = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }
    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits: Vec<&mut Limit> = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));
        limits
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                },
                BidOrAsk::Ask => match self.bids.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            
        }
    } 
}

#[derive(Debug)]

pub struct Limit {
    price: Decimal,
    orders: Vec<Order>,
}
impl Limit {
    pub fn new(price: Decimal) -> Limit {
        Limit { 
            price,
            orders: Vec::new(),
        }
    }
    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                },
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                },
            }
            if market_order.is_filled() {
                break;
            }
        }
    }
    fn total_volume(&self) -> f64 {
        let volume: f64 = self
            .orders
            .iter()
            .map(|order| order.size)
            .reduce(|a,b| a + b)
            .unwrap();
        volume
    }

}
#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}
impl Order{
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order {bid_or_ask, size}
    }
    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

