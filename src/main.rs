use std::fs::File;
use std::env;
use std::io::Read;

mod one;

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem = args[1].as_str();
    let filename = &args[2];

    let input = &mut String::new();
    let f = File::open(filename)
        .and_then(|mut f| { f.read_to_string(input) });

    if f.is_err() {
        println!("Couldn't read file {}", filename);
        return;
    }

    match problem {
        "one" => one::solve(input),
        _ => println!("Dunno mate")
    };
}

