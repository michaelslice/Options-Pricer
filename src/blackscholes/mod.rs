use std::io;

pub struct BlackScholes {
    pub option_price: f32, 
    pub stock_price: f32,
    pub strike_price: f32,
    pub risk_free_interest_rate: f32,
    pub time_to_maturity: u32,
    pub normal_distribution: f32,
}
impl BlackScholes {

    pub fn price_option(&self) ->u32 {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("FAILED to read line");
        println!("you entered {}", input);
        
        return 12
    }
    
    


}

