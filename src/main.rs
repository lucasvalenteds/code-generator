mod generator;

use generator::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() == 2 {
        let argument: String = args[1].to_owned();

        if argument.len() == 2 {
            match generate_code(Some(&argument), None) {
                Some(code) => println!("{}", code),
                None => panic!("Could not generate code"),
            }
        } else {
            match generate_code(None, Some(&argument)) {
                Some(code) => println!("{}", code),
                None => panic!("Could not generate code"),
            }
        }
    } else {
        match generate_code(None, None) {
            Some(code) => println!("{}", code),
            None => panic!("Could not generate code"),
        }
    }
}
