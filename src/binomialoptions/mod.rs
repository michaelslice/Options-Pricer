use std::io;
use std::num::*;

pub struct BinomialOptions {
    pub option_price: f64, 
    pub stock_price: f64,
    pub strike_price: f64,
    pub risk_free_interest_rate: f64,
    pub time_to_maturity: f64,
    pub volatility: f64,
}
impl BinomialOptions {

    pub fn build_model(
        option_price: f64, 
        stock_price: f64,
        strike_price: f64,
        risk_free_interest_rate: f64,
        time_to_maturity: f64,
        volatility: f64
    ) -> Self {
        BinomialOptions {
        option_price,
        stock_price,
        strike_price,
        risk_free_interest_rate,
        time_to_maturity,
        volatility
        }
    }
/*
    pub fn price_option(&self) -> f64 {
    
    let natural_log_base: f64 = 2.71828;

    let d1: f64 = (self.stock_price / self.strike_price) 
        + (self.risk_free_interest_rate + (self.volatility.powf(2.0) / 2.0))
            / ((self.volatility as f64) * (self.time_to_maturity as f64).sqrt());
    
    let d2: f64 = d1 - self.volatility * self.time_to_maturity;

    let c: f64 = d1 * self.stock_price - d2 * self.strike_price * natural_log_base.powf(self.risk_free_interest_rate * self.time_to_maturity); 

        c
    }
 */
}
