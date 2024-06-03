use std::io;
use crate::blackscholes::BlackScholes;
use crate::binomialoptions::BinomialOptions;

pub fn prompt() {
    
    loop {
        println!("Do you want to use the Black Scholes Model, or Binomial Options Pricing Model?(1 for Black Scholes, 2 for Binomial Options, 3 to quit)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("FAILED to read line");
    
        match input.trim().parse::<i16>() {    
            Ok(number) => {
                if number == 1 || number == 2 {                
                    if number == 1 {

                        // Black Scholes Model
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

                        let option_contract = BlackScholes::build_model(
                            _stock_price_num,
                            _strike_price_num,
                            _risk_free_interest_rate_num,
                            _time_to_maturity_num,
                            _volatility
                        );

                        let ans: f64 = BlackScholes::price_option_call(&option_contract);
                        println!("The Option Price Call is ${:.2}", ans);
                            
                        let ans: f64 = BlackScholes::price_option_put(&option_contract);
                        println!("The Option Price Put is ${:.2}", ans);   

                    }
                    else {

                        // Binomial Option Model
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
                        
                        println!("Enter the Number of Steps for the Binomial Model");
                        let mut _number_of_steps = String::new();
                        io::stdin().read_line(&mut _number_of_steps).expect("FAILED to read line");   
            
                        println!("Enter the Option Type, C or P (C for Call, P for Put");
                        let mut _option_type = String::new();
                        io::stdin().read_line(&mut _option_type).expect("FAILED to read line");   
            
                        println!("Enter the Up Factor");
                        let mut _up_factor = String::new();
                        io::stdin().read_line(&mut _up_factor).expect("FAILED to read line");   
        
                        println!("Enter the Down Factor");
                        let mut _down_factor = String::new();
                        io::stdin().read_line(&mut _down_factor).expect("FAILED to read line");   

                        let _stock_price_num: f64 = _stock_price.trim().parse().expect("Failed to parse input as u32");
                        let _strike_price_num: f64 = _strike_price.trim().parse().expect("Failed to parse input as u32");
                        let _risk_free_interest_rate_num: f64 = _risk_free_interest_rate.trim().parse().expect("Failed to parse input as u32");
                        let _time_to_maturity_num: f64 = _time_to_maturity.trim().parse().expect("Failed to parse input as u32");
                        let _number_of_steps_num: usize = _number_of_steps.trim().parse().expect("Failed to parse input as u32");                    
                        let _option_type_char: char = _option_type.trim().parse().expect("Failed to parse input as char");
                        let _up_factor_num: f64 = _up_factor.trim().parse().expect("Failed to parse input as u32");                    
                        let _down_factor_num: f64 = _down_factor.trim().parse().expect("Failed to parse input as u32"); 

                        let option_contract = BinomialOptions::build_model(
                            _stock_price_num,
                            _strike_price_num,
                            _risk_free_interest_rate_num,
                            _time_to_maturity_num,
                        );         
                        
                        let ans: f64 = BinomialOptions::binomial_option_model(
                            &option_contract
                            , _number_of_steps_num
                            , _option_type_char
                            , _up_factor_num
                            , _down_factor_num
                        );
                        println!("The Option Price Call is ${:.2}", ans);
                        
                    }
                }
                else if number  == 3 {
                    break;
                }
                
                else {
                    println!("Error you entered {}, please enter either 1, 2, or 3", input);
            
                }
            },
            Err(_) => println!("ERROR Unable to parse num"),
        }
    }
}