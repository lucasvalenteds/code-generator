mod generator;

use generator::*;

fn main() {
    for _ in 0..20 {
        match generate_code(None, None) {
            Some(code) => println!("{}", code),
            None => panic!("Could not generate code"),
        }
    }
}
