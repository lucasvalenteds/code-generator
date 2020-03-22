pub fn generate_code(seed: Option<&str>) -> Option<String> {
    let code: &str = match seed {
        Some(it) => it,
        None => "0011001100110011",
    };

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

    Some(format!("{}{}{}", code, first_digit, second_digit))
}

#[cfg(test)]
mod tests {

    use super::generate_code;

    #[test]
    fn test_seed_valid() {
        assert_eq!(
            generate_code(Some("00124b00188a7f68")).as_deref(),
            Some("00124b00188a7f6810").as_deref()
        );
    }

    #[test]
    fn test_seed_too_short() {
        assert_eq!(generate_code(Some("00124b00188a7f6")).as_deref(), None);
    }

    #[test]
    fn test_seed_too_long() {
        assert_eq!(generate_code(Some("00124b00188a7f689")).as_deref(), None);
    }

    #[test]
    fn test_no_seed() {
        assert_ne!(generate_code(None).as_deref(), None);
    }
}
