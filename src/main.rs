mod generator;

fn main() {
    match generator::generate_code(None) {
        Some(code) => println!("{}", code),
        None => panic!("Could not generate code"),
    }
}
