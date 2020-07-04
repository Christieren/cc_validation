// Example fake numbers to test with: Visa: 4716718003943986 / Master Card: 5159262821225525 / AMEX: 341752169978719
use std::io;
fn main() {
    let mut cc_number = String::new();

    println!("Enter a credit card number:");
    io::stdin()
        .read_line(&mut cc_number)
        .expect("Failed to read line!");
    cc_number = cc_number.trim().to_string();
    if cc_number.len() != 0{
        println!("Number entered: {}", cc_number);
        if validate_cc(&cc_number){
            println!("Number is valid!");
        }
        else{
            println!("Number is invalid...");
        }
    }
    else{
        println!("No number entered!")
    }
}

fn validate_cc(cc_number: &str) -> bool{

    let n_digits: usize = cc_number.chars().count();
   
    let cc_sum_opt: Option<u32> = cc_number.chars().last().unwrap().to_digit(10);
    let mut cc_sum: u32 = 0;

    match cc_sum_opt {
        Some(x) => cc_sum = x,
        None => println!("Error converting sum char")
    }
    assert_ne!(cc_sum_opt, None);

    let parity : usize = n_digits % 2;
    let num_max = 9;

    for (i, c) in cc_number.chars().enumerate() {
        let mut digit: u32 = match c.to_digit(10) {
            Some(num) => num,
            None => break,
        };
        if i == n_digits - 1{
            break;
        }
        if i % 2 == parity {
            digit = digit * 2;
        }
        if digit > num_max {
            digit = digit - num_max;
        }
        if i < n_digits - 1{
            cc_sum += digit;
        }
    }
        return (cc_sum % 10) == 0;
}
