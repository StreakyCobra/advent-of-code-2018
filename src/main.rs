use std::env;
use std::process;

mod utils;
mod prob_01;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error, wrong number of arguments.\n");
        println!("Usage: {} <problem_number>", args[0]);
        process::exit(1);
    }
    let problem: Option<u8> = args[1].parse::<u8>().ok();
    match problem {
        Some(1) => prob_01::solve(),
        Some(_) | None => {
        println!("Problem solver not implemented yet.");
        process::exit(1);
        }
    }
}
