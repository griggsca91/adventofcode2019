use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_name = &args[1];

    let file = fs::File::open(file_name).expect("something wrong happened");
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    for (index, line) in reader.lines().enumerate() {
        let mass = line.unwrap().parse::<i32>().unwrap();

        let fuel_needed = (mass / 3) - 2;
        total_fuel += fuel_needed;

    }

    println!("{}", total_fuel);
    
}
