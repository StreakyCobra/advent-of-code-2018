extern crate chrono;

use std::env;
use std::process;

mod prob_01;
mod prob_02;
mod prob_03;
mod prob_04;
mod utils;

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
        Some(2) => prob_02::solve(),
        Some(3) => prob_03::solve(),
        Some(4) => prob_04::solve(),
        Some(_) | None => {
            println!("Problem solver not implemented yet.");
            process::exit(1);
        }
    }
}
