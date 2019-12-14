use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];


    let contents = fs::read_to_string(file_name).expect("Ruh roh");

    let input: Vec<i32> = contents.split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();
    println!("{:?}", input);
    process_input(input);
}

fn process_input(mut code: Vec<i32>) {

    let mut pc: usize = 0;

    while pc < code.len() {
        match code[pc] {
            // Adding opcode
            1 => {
                println!("Adding");
                let first = code[code[pc + 1] as usize];
                let second = code[code[pc + 2] as usize];
                println!("Adding: {} + {}", first, second);
                let destination = code[pc + 3] as usize;
                println!("Overriding: {}", code[pc + 3]);
                code[destination] = first + second;
                pc = pc + 4;
            }
            // Multiplication
            2 => {
               println!("Multiplication");
                let first = code[code[pc + 1] as usize];
                let second = code[code[pc + 2] as usize];
                println!("Multiplying: {} + {}", first, second);
                let destination = code[pc + 3] as usize;
                println!("Overriding: {}", code[pc + 3]);
                code[destination] = first * second;
                pc = pc + 4;
            }
            99 |_ => break
        }
        println!("{:?}", code);
    }

    println!("Ended parsing at pc: {}", pc);

    println!("{:?}", code);
}
