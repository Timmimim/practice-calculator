// src/main.rs

/**
 * Calculator inspired by https://medium.com/@danmugh/rust-for-beginners-dive-into-coding-with-these-5-projects-to-boost-your-skills-7307e7923d74
 * Extended by some options, structure, overly ambitious (and currently inactive) templating.
 */

use std::{io,process};

use practice_calculator::Calculator;

fn main() {
    println!("Simple Calculator");
    println!("1. Addition:\t\tadd");
    println!("2. Subtraction:\t\tsub");
    println!("3. Multiplication\tmult");
    println!("4. Division:\t\tdiv");
    println!("5. Modulo:\t\tmod");
    println!("6. Factorial:\t\tfac");
    println!("7. Square:\t\tsq");
    println!("8. Exponentiation:\texp");
    println!("9. Square Root:\t\tsqrt");
    println!("0. Exit:\t\tquit\n");
    loop {
        println!("Option: ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice : &str = choice.trim();
        if choice.is_empty() {continue;}

        match choice {
            "add"   => {
                println!("Addition:\tX + Y");
            }
            "sub"   => {
                println!("Subtraction:\tX - Y");
            }
            "mult"  => {
                println!("Multiplication:\tX * Y");
            }
            "div"   => {
                println!("Division:\tX / Y");
            }
            "mod"   => {
                println!("Modulo:\tX mod Y");
            }
            "exp"   => {
                println!("Exponentiation:\tX ^ Y");
            }
            "fac"   => {
                println!("Factorial:\tX!");
            }
            "sq"    => {
                println!("Square:\tX ^ 2");
            }
            "sqrt"  => {
                println!("Square Root:\tsqrt(X)");
            }
            "quit" => {
                println!("Exiting. Goodbye!");
                process::exit(0);
            }
            _ => {
                println!("Somehow, an invalid option made it through:{}\nPlease try again or contact the author.\n", choice);
                continue;
            }
        };

        let calc : Calculator =  match choice {
            "add" | "sub" | "mult" | "div" | "mod" | "exp" => {
                println!("Enter the first numeric parameter of the equation (X):");
                let mut num1 = String::new();
                io::stdin().read_line(&mut num1).expect("Failed to read line");
                let num1:f64 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        continue;
                    }
                };
                println!("Enter the second numeric parameter of the equation (Y):");
                let mut num2 = String::new();
                io::stdin().read_line(&mut num2).expect("Failed to read line");
                let num2:f64 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        continue;
                    }
                };
                Calculator::new(num1, Some(num2))
            }
            "fac" | "sq" | "sqrt" => {
                println!("Enter the numeric parameter of the equation (X):");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Failed to read line");
                let num:f64 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        continue;
                    }
                };
                Calculator::new(num, None)
            }
            _ => {
                println!("Invalid input. Please enter a valid option.");
                continue;
            }
        };

        match choice {
            "add"   => {
                let result = calc.add();
                println!("Result: {}", result);
            }
            "sub"   => {
                let result = calc.subtract();
                println!("Result: {}", result);
            }
            "mult"  => {
                let result = calc.multiply();
                println!("Result: {}", result);
            }
            "div"   => {
                match calc.divide() {
                    Err(why) => panic!("Dividing {} by {} resulted in Error: {:?}", calc.get_param_a(), calc.get_param_b(), why),
                    Ok(result) => println!("Result: {}", result),
                }
            }
            "mod"   => {
                let result = calc.modulo();
                match result {
                    Ok(res) => println!("Result: {}", res),
                    Err(err) => println!("Result: {}", err),
                };
            }
            "exp"   => {
                let result = calc.pow();
                println!("Result: {}", result);
            }
            "fac"   => {
                let result = calc.facto();
                println!("Result: {}", result);
            }
            "sq"    => {
                let result = calc.square();
                println!("Result: {}", result);
            }
            "sqrt"  => {
                match calc.sqrt() {
                    Err(why) => panic!("Drawing the Square Root of {} resulted in Error: {:?}", calc.get_param_a(), why),
                    Ok(result) => println!("Result: {}", result),
                }

            }
            _ => {
                println!("Somehow, an invalid option made it through: {}\nPlease try again or contact the author.", choice);
            }
        };

        println!("\nWould you like to perform another calculation?");
    }
}
