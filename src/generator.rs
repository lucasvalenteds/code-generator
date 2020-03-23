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

fn get_code_type_value(code_type: Option<&str>) -> String {
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

pub fn generate_code(code_type: Option<&str>, seed: Option<&str>) -> Option<String> {
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
        assert_eq!(
            generate_code(Some("00"), Some("00124b00188a7f68")).as_deref(),
            Some("00124b00188a7f6810").as_deref()
        );
    }

    #[test]
    fn test_seed_too_short() {
        assert_eq!(
            generate_code(None, Some("00124b00188a7f6")).as_deref(),
            None
        );
    }

    #[test]
    fn test_seed_too_long() {
        assert_eq!(
            generate_code(None, Some("00124b00188a7f689")).as_deref(),
            None
        );
    }

    #[test]
    fn test_no_seed() {
        assert_ne!(generate_code(None, None).as_deref(), None);
    }
}
