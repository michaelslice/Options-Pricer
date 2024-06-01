
pub struct BinomialOptions {
    pub stock_price: f64,
    pub strike_price: f64,
    pub risk_free_interest_rate: f64,
    pub time_to_maturity: f64,
}
impl BinomialOptions {

    pub fn build_model(
        stock_price: f64,
        strike_price: f64,
        risk_free_interest_rate: f64,
        time_to_maturity: f64,
    ) -> Self {
        BinomialOptions {
        stock_price,
        strike_price,
        risk_free_interest_rate,
        time_to_maturity,
        }
    }
    
    pub fn binomial_option_model(
        &self,
        number_of_steps: usize,
        option_type: char,
        up_factor: f64,
        down_factor: f64,
    ) -> f64 {

        let dt = self.time_to_maturity / number_of_steps as f64;
        let q = ((self.risk_free_interest_rate * dt).exp() - down_factor) / (up_factor - down_factor);
        let disc = (-self.risk_free_interest_rate * dt).exp();
    
        let mut c: Vec<f64> = Vec::with_capacity(number_of_steps + 1);
    
        for i in 0..=number_of_steps {
            c.push(self.stock_price * down_factor.powi((number_of_steps - i) as i32) * up_factor.powi(i as i32));
        }
    
        for i in 0..=number_of_steps {
            c[i] = c[i].max(0.0);
        }
    
        match option_type {
            'C' => c.iter_mut().for_each(|val| *val = val.max(0.0)),
            'P' => c.iter_mut().for_each(|val| *val = val.max(0.0 - self.strike_price)),
            _ => panic!("Invalid option type, Only C(CALL), or P(PUT) are Valid Options.")
        }   

        for i in (1..=number_of_steps).rev() {
            for j in 0..i { 
                c[j] = disc * (q * c[j + 1] + (1.0 - q) * c[j]);
            }
        }
    
        c[0]
    }
}