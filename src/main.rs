mod day1;
mod day2;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a day number");
        return;
    }

    let day: u32 = args[1].parse().expect("Please provide a valid day number");

    match day {
        1 => day1::run(),
        2 => day2::run(),
        _ => println!("Day {} not implemented yet", day),
    }
}
