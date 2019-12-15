use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let contents = fs::read_to_string(file_name).expect("Ruh roh");
    let range: Vec<&str> = contents.split("-").map(|s| s.trim()).collect();
    let mut current = String::from(range[0].clone());
    println!("{:?}", range);
    let mut total = 0;
    while current != range[1] {
        current = increment(&current);
        if is_potential(&current) {
           total += 1; 
        }
    }
    println!("Total: {}", total);
}

fn is_potential(s_number: &String) -> bool {
    println!("");
    println!("is_potential: {}", s_number);
    let chars = s_number.as_bytes();

    let mut has_repeating = false;
    // all numbers are either increasing or equal to the next one
    for (i, character) in s_number.chars().enumerate() {
        if i+1 == s_number.len() {
            break;
        }
        let first: u32 = character.to_digit(10).unwrap();
        let second: u32 = (chars[i+1] as char).to_digit(10).unwrap();
        println!("======");
        println!("{} > {} = {}", first, second, first > second);
        if first > second {
            println!("returning false");
            return false
        }

        has_repeating |= first == second;
        println!("Has repeating: {}", has_repeating);
    }


    println!("*****************");
    has_repeating
}

fn increment(s_number: &str) -> String {
    let number: i32 = s_number.parse().unwrap();

    (number+1).to_string()
}
