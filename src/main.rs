mod generator;

fn main() {
    let seed = Some("00124b00188a7f68");
    if let Some(code) = generator::generate_code(seed) {
        println!("{}", code)
    } else {
        println!("Could not generate code")
    }

    if let Some(code) = generator::generate_code(None) {
        println!("{}", code)
    } else {
        println!("Could not generate code")
    }
}
