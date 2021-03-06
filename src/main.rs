mod generator;

use generator::*;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let (code_type, seed) = match args.len() {
        2 => match args[1].len() {
            2 => (Some(args[1].to_owned()), None),
            _ => (None, Some(args[1].to_owned())),
        },
        _ => (None, None),
    };

    if let Some(code) = generate_code(code_type, seed) {
        println!("{}", code);
        exit(0);
    } else {
        println!("Could not generate the code");
        exit(1);
    }
}
