// mod blackscholes;
use std::io;



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
                        blackscholes::price_option();
                  
                    
                    }
                    else {
                        println!("RUNNING BINOMIAL METHOD");
                        binomialoptions::price_option();
                    
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
