mod generator;

use generator::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let (code_type, seed) = match args.len() {
        2 => match args[1].len() {
            2 => (Some(args[1].to_owned()), None),
            _ => (None, Some(args[1].to_owned())),
        },
        _ => (None, None),
    };

    match generate_code(code_type, seed) {
        Some(code) => println!("{}", code),
        None => println!("Could not generate code"),
    }
}
