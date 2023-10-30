use super::orderbook::{Order,Orderbook};
use std::collections::HashMap;
use rust_decimal::prelude::*;
#[derive(Debug, Eq, PartialEq, Hash, Clone)]

pub struct TradingPair {
    base: String,
    quote: String,

}
impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote
        }
    }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote) 
    }
}
pub struct MatchingEngine {
    //holds the orderbook
    orderbooks: HashMap<TradingPair, Orderbook>
}
impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }
    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("Opening a new orderbook for market {:?}", pair.to_string())
    }
    pub fn place_limit_order(&mut self, pair: TradingPair, price: Decimal, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                println!("Limit order filled at price {:?}", price);
                Ok(())
            }
            None => Err(format!(
                "The orderbook for the given trading pair ({}) does not exist.",
            pair.to_string()
            )),
            
        }
    }
}