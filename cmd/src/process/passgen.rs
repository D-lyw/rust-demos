use anyhow::Error;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn password_generate(
    length: u8,
    number: bool,
    uppercase: bool,
    lowercase: bool,
    symbol: bool,
) -> Result<String, Error> {
    let mut password = Vec::new();
    let mut chars = Vec::new();
    if number {
        let number_chars = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        password.push(number_chars[thread_rng().gen_range(0..number_chars.len())]);
        chars.extend(number_chars);
    }
    if lowercase {
        let lowercase_chars = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't',
            'u', 'v', 'w', 'x', 'y', 'z',
        ];
        password.push(lowercase_chars[thread_rng().gen_range(0..lowercase_chars.len())]);
        chars.extend(lowercase_chars);
    }
    if uppercase {
        let uppercase_chars = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T',
            'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        password.push(uppercase_chars[thread_rng().gen_range(0..uppercase_chars.len())]);
        chars.extend(uppercase_chars);
    }
    if symbol {
        let symbol_chars = vec!['@', '#', '$', '%', '&', '*', '_', '-', '+', '='];
        password.push(symbol_chars[thread_rng().gen_range(0..symbol_chars.len())]);
        chars.extend(symbol_chars);
    }

    for _ in 0..(length - password.len() as u8) {
        password.push(chars[thread_rng().gen_range(0..chars.len())]);
    }
    password.shuffle(&mut thread_rng());

    Ok(String::from_iter(password))
}
