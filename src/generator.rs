use rand::distributions::Distribution;
use rand::{seq::SliceRandom, thread_rng, Rng};

struct Hexadecimal;

impl Distribution<char> for Hexadecimal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        *b"0123456789abcdef".choose(rng).unwrap() as char
    }
}

struct Decimal;

impl Distribution<char> for Decimal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        *b"0123456789".choose(rng).unwrap() as char
    }
}

fn get_code_type_value(code_type: Option<String>) -> String {
    match code_type {
        Some(digits) => digits.to_string(),
        None => thread_rng()
            .sample_iter(&Decimal)
            .take(2)
            .collect::<String>(),
    }
}

fn get_random_code() -> String {
    thread_rng()
        .sample_iter(&Hexadecimal)
        .take(14)
        .collect::<String>()
}

pub fn generate_code(code_type: Option<String>, seed: Option<String>) -> Option<String> {
    let code: String = match seed {
        Some(it) => String::from(it),
        None => format!("{}{}", get_code_type_value(code_type), get_random_code()),
    };

    let digits: Vec<u32> = code
        .chars()
        .filter_map(|character: char| character.to_digit(16))
        .collect();

    if digits.len() != 16 {
        return None;
    }

    let first_digit: u32 = digits
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, digit)| ((index as u32) + 1) * digit)
        .sum();

    let second_digit: u32 = digits
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, digit)| (index as u32) * digit)
        .sum();

    Some(format!(
        "{}{:x}{:x}",
        code,
        first_digit % 16,
        second_digit % 16
    ))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_seed_valid() {
        let code_type = Some("00".to_string());
        let seed = Some("00124b00188a7f68".to_string());

        assert_eq!(
            generate_code(code_type, seed).as_deref(),
            Some("00124b00188a7f6810").as_deref()
        );
    }

    #[test]
    fn test_seed_too_short() {
        let code_type = None;
        let seed = Some("00124b00188a7f6".to_string());

        assert_eq!(generate_code(code_type, seed).as_deref(), None);
    }

    #[test]
    fn test_seed_too_long() {
        let code_type = None;
        let seed = Some("00124b00188a7f689".to_string());

        assert_eq!(generate_code(code_type, seed).as_deref(), None);
    }

    #[test]
    fn test_no_seed() {
        let code_type = None;
        let seed = None;

        assert_ne!(generate_code(code_type, seed).as_deref(), None);
    }
}
