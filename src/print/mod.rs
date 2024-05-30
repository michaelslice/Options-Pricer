use std::io;
use crate::blackscholes::BlackScholes;


pub fn prompt() {
    
    loop {
        println!("Do you want to use the Black Scholes Model, or Binomial Options Pricing Model?(1 for Black Scholes, 2 for Binomial Options, 3 to quit)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("FAILED to read line");
    
        match input.trim().parse::<i16>() {    
            Ok(number) => {
                if number == 1 || number == 2 {                
                    if number == 1 {
                        println!("RUNNG BLACK SCHOLES");
                    
                        println!("Enter a Option Price");
                        let mut _option_price = String::new();
                        io::stdin().read_line(&mut _option_price).expect("FAILED to read line");
                    
                        println!("Enter a Stock Price");
                        let mut _stock_price = String::new();
                        io::stdin().read_line(&mut _stock_price).expect("FAILED to read line");
                    
                        println!("Enter a Strike Price");
                        let mut _strike_price = String::new();
                        io::stdin().read_line(&mut _strike_price).expect("FAILED to read line");
                    
                        println!("Enter a Risk Free Interest Rate Price");
                        let mut _risk_free_interest_rate = String::new();
                        io::stdin().read_line(&mut _risk_free_interest_rate).expect("FAILED to read line");
                    
                        println!("Enter the Time to Maturity Price");
                        let mut _time_to_maturity = String::new();
                        io::stdin().read_line(&mut _time_to_maturity).expect("FAILED to read line");
                        
                        println!("Enter the Normal Distribution");
                        let mut _normal_distribution = String::new();
                        io::stdin().read_line(&mut _normal_distribution).expect("FAILED to read line");   

                        let _option_price_num: u32 = _option_price.trim().parse().expect("Failed to parse input as u32");
                        let _stock_price_num: u32 = _stock_price.trim().parse().expect("Failed to parse input as u32");
                        let _strike_price_num: u32 = _strike_price.trim().parse().expect("Failed to parse input as u32");
                        let _risk_free_interest_rate_num: u32 = _risk_free_interest_rate.trim().parse().expect("Failed to parse input as u32");
                        let _time_to_maturity_num: u32 = _time_to_maturity.trim().parse().expect("Failed to parse input as u32");
                        let _normal_distribution_num: u32 = _normal_distribution.trim().parse().expect("Failed to parse input as u32");

                        let user = BlackScholes::build_model(
                            _option_price_num, 
                            _stock_price_num,
                            _strike_price_num,
                            _risk_free_interest_rate_num,
                            _time_to_maturity_num,
                            _normal_distribution_num
                        );

 
                            

                     
                    
                    }
                    else {
                        println!("RUNNING BINOMIAL METHOD");
                        
                        
                        
                     
                    }
                    
                    // break;
                }
                else {
                    println!("ERRROR you entered {}", input);
            
                }
            },
            Err(_) => println!("ERROR Unable to parse num"),
        }
    }
}
