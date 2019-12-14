use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];


    let contents = fs::read_to_string(file_name).expect("Ruh roh");

    let input: Vec<i32> = contents.split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();
    println!("{:?}", input);

    for noun in 0..100 {
        for verb in 0..100 {
            println!("Trying Noun: {}, Verb: {}", noun, verb);
            let mut commands = input.clone();
            commands[1] = noun as i32;
            commands[2] = verb as i32;
            let output = process_input(commands);
            println!("Output: {}", output);
            if output >= 19690720 {
                println!("Found Noun: {}, Verb: {}", noun, verb);
                return
            }
        }
    }


}

fn process_input(mut code: Vec<i32>) -> i32 {

    let mut pc: usize = 0;

    while pc < code.len() {
        match code[pc] {
            // Adding opcode
            1 => {
                let first = code[code[pc + 1] as usize];
                let second = code[code[pc + 2] as usize];
                let destination = code[pc + 3] as usize;
                code[destination] = first + second;
                pc = pc + 4;
            }
            // Multiplication
            2 => {
                let first = code[code[pc + 1] as usize];
                let second = code[code[pc + 2] as usize];
                let destination = code[pc + 3] as usize;
                code[destination] = first * second;
                pc = pc + 4;
            }
            99 |_ => break
        }
    }

    code[0]
}
