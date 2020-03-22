// 00124b00188a7f6810

fn generate_code(code: &str) -> Option<String> {
    if code.len() != 16 {
        return None;
    }

    let characters: Vec<char> = code.chars().collect();

    let mut characters_as_hex: Vec<u32> = Vec::with_capacity(16);
    for index in 0..16 {
        if let Some(character_as_hex) = characters[index].to_digit(16) {
            characters_as_hex.push(character_as_hex);
        } else {
            return None;
        }
    }

    let mut first_digit_sequence: Vec<u32> = Vec::with_capacity(16);
    for index in 0..characters_as_hex.len() {
        first_digit_sequence.push(characters_as_hex[index] * (index + 1) as u32);
    }
    let first_digit: u32 = (first_digit_sequence.iter().sum::<u32>()) % 16;

    let mut second_digit_sequence: Vec<u32> = Vec::with_capacity(16);
    for index in 0..characters_as_hex.len() {
        second_digit_sequence.push(characters_as_hex[index] * index as u32);
    }
    let second_digit: u32 = (second_digit_sequence.iter().sum::<u32>()) % 16;

    characters_as_hex.push(first_digit);
    characters_as_hex.push(second_digit);

    println!("{:?}", second_digit_sequence);
    println!("{}", first_digit);
    println!("{}", second_digit);

    Some(format!("{}{}{}", code, first_digit, second_digit))
}

fn main() {
    let seed = "00124b00188a7f68";

    if let Some(code) = generate_code(seed) {
        println!("{}", code)
    } else {
        println!("Could not generate code")
    }
}
