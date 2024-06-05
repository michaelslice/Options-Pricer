use statrs::distribution::{Normal, ContinuousCDF};

pub struct BlackScholes {
    pub stock_price: f64,
    pub strike_price: f64,
    pub risk_free_interest_rate: f64,
    pub time_to_maturity: f64,
    pub volatility: f64,
}
impl BlackScholes {

    pub fn build_model(
        stock_price: f64,
        strike_price: f64,
        risk_free_interest_rate: f64,
        time_to_maturity: f64,
        volatility: f64
    ) -> Self {
        BlackScholes {
        stock_price,
        strike_price,
        risk_free_interest_rate,
        time_to_maturity,
        volatility
        }
    }
    
    pub fn price_option_call(&self) -> f64 {
    
        let d1: f64 = (self.stock_price / self.strike_price).ln()
            + (self.risk_free_interest_rate + (self.volatility.powf(2.0) / 2.0)) * self.time_to_maturity
            / (self.volatility * self.time_to_maturity.sqrt());
         
        let d2: f64 = d1 - self.volatility * self.time_to_maturity.sqrt();

        let standard_normal = Normal::new(0.0, 1.0).unwrap();
        let cdf_d1 = standard_normal.cdf(d1);
        let cdf_d2 = standard_normal.cdf(d2);

        let c: f64 = self.stock_price * cdf_d1 - self.strike_price * (-self.risk_free_interest_rate * self.time_to_maturity).exp() * cdf_d2; 

        c
    }  

    pub fn price_option_put(&self) -> f64 {
        
        let d1: f64 = (self.stock_price / self.strike_price).ln()
            + (self.risk_free_interest_rate + (self.volatility.powf(2.0) / 2.0)) * self.time_to_maturity
            / (self.volatility * self.time_to_maturity.sqrt());
    
        let d2: f64 = d1 - self.volatility * self.time_to_maturity.sqrt();

        let standard_normal = Normal::new(0.0, 1.0).unwrap();
        let cdf_d1 = standard_normal.cdf(-d1);
        let cdf_d2 = standard_normal.cdf(-d2);

        let p = self.strike_price * (-self.risk_free_interest_rate * self.time_to_maturity).exp() * cdf_d2
            - self.stock_price * cdf_d1;

        p
    }

    pub fn price_delta_call(&self) -> f64 {
        
        12.0
    }   

    pub fn price_delta_put(&self) -> f64 {
        
        12.0
    } 
}