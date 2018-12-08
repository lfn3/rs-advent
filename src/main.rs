extern crate core;

use std::env;
use std::fs::File;
use std::io::Read;

mod one;
mod two;
mod five;

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem = &args[1];
    let filename = &format!("inputs/{}.txt", problem);
    let input = &mut String::new();

    let f = File::open(filename).and_then(|mut f| f.read_to_string(input));

    if f.is_err() {
        println!("Couldn't read file {}", filename);
        return;
    }

    match problem.as_str() {
        "one" => one::solve(input),
        "two" => two::solve(input),
        "five" => five::solve(input),
        _ => println!("Dunno mate"),
    };
}
