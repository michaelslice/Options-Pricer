use std::io;

pub struct BinomialOptions {
    pub option_price: u32, 
    pub stock_price: u32,
    pub strike_price: u32,
    pub risk_free_interest_rate: u32,
    pub time_to_maturity: u32,
    pub normal_distribution: u32,
}
impl BinomialOptions {

    pub fn build_model(
        option_price: u32, 
        stock_price: u32,
        strike_price: u32,
        risk_free_interest_rate: u32,
        time_to_maturity: u32,
        normal_distribution: u32
    ) -> BinomialOptions {
        BinomialOptions {
        option_price,
        stock_price,
        strike_price,
        risk_free_interest_rate,
        time_to_maturity,
        normal_distribution
        }
    }
    
    
    
    pub fn price_option(&self) ->u32 {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("FAILED to read line");
        println!("you entered {}", input);
        
        return 12
    }
    
    


}
