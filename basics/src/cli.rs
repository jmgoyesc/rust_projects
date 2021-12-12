use std::env;

pub fn run() {
    println!("cli");
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("Arguments: {:?}", args);
    println!("Command: {}", command);
}