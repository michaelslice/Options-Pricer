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
                        println!("Enter the Stock Price");
                        let mut _stock_price = String::new();
                        io::stdin().read_line(&mut _stock_price).expect("FAILED to read line");
                    
                        println!("Enter the Strike Price");
                        let mut _strike_price = String::new();
                        io::stdin().read_line(&mut _strike_price).expect("FAILED to read line");
                    
                        println!("Enter the Risk-Free Interest Rate (annual)");
                        let mut _risk_free_interest_rate = String::new();
                        io::stdin().read_line(&mut _risk_free_interest_rate).expect("FAILED to read line");
                    
                        println!("Enter the Time to Expiration (in years)");
                        let mut _time_to_maturity = String::new();
                        io::stdin().read_line(&mut _time_to_maturity).expect("FAILED to read line");
                        
                        println!("Enter the Stock Volatility (annualized)");
                        let mut _volatility = String::new();
                        io::stdin().read_line(&mut _volatility).expect("FAILED to read line");   
            
                        let _stock_price_num: f64 = _stock_price.trim().parse().expect("Failed to parse input as u32");
                        let _strike_price_num: f64 = _strike_price.trim().parse().expect("Failed to parse input as u32");
                        let _risk_free_interest_rate_num: f64 = _risk_free_interest_rate.trim().parse().expect("Failed to parse input as u32");
                        let _time_to_maturity_num: f64 = _time_to_maturity.trim().parse().expect("Failed to parse input as u32");
                        let _volatility: f64 = _volatility.trim().parse().expect("Failed to parse input as u32");

                        let user = BlackScholes::build_model(
                            _stock_price_num,
                            _strike_price_num,
                            _risk_free_interest_rate_num,
                            _time_to_maturity_num,
                            _volatility
                        );

                        let mut ans: f64 = BlackScholes::price_option_call(&user);
                        println!("The Option Price Call is ${:.2}", ans);
                            
                        let mut ans: f64 = BlackScholes::price_option_put(&user);
                        println!("The Option Price Put is ${:.2}", ans);
                            
                    }
                    else {
                        println!("RUNNING BINOMIAL METHOD");
                        
                        
                        
                     
                    }
                    
                    // break;
                }
                else {
                    println!("Error you entered {}, please enter either 1, 2, or 3", input);
            
                }
            },
            Err(_) => println!("ERROR Unable to parse num"),
        }
    }
}
