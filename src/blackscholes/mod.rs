use std::io;

pub struct BlackScholes {
    pub option_price: f64, 
    pub stock_price: f64,
    pub strike_price: f64,
    pub risk_free_interest_rate: f64,
    pub time_to_maturity: f64,
    pub volatility: f64,
}
impl BlackScholes {

    pub fn build_model(
        option_price: f64, 
        stock_price: f64,
        strike_price: f64,
        risk_free_interest_rate: f64,
        time_to_maturity: f64,
        volatility: f64
    ) -> Self {
        BlackScholes {
        option_price,
        stock_price,
        strike_price,
        risk_free_interest_rate,
        time_to_maturity,
        volatility
        }
    }
    
    
    pub fn price_option(&self) {

  
    }   
}