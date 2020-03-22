mod generator;

use generator::*;

fn main() {
    match generate_code(CodeType::Type0, None) {
        Some(code) => println!("{}", code),
        None => panic!("Could not generate code"),
    }
}
