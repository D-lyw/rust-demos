use thiserror::Error;

const BASE32_ALPHABET: [u8; 32] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'2', b'3', b'4', b'5', b'6', b'7',
];

#[derive(Debug, Error)]
enum Base32Error {
    #[error("Not valid base32 chat: {0}")]
    InvalidCharError(char),
    #[error("Length error, out of decode length border")]
    DecodeLengthError,
}

fn base32_encode(input: &[u8]) -> String {
    let mut bits = 0u64;
    let mut bits_len = 0;
    let mut output = String::new();

    for &byte in input.iter() {
        println!("{:x}", bits << 8);
        bits = (bits << 8) | (byte as u64);
        bits_len += 8;
        println!("bits: {:x}", bits);

        while bits_len >= 5 {
            bits_len -= 5;
            let index = (bits >> bits_len) & 0b11111;
            output.push(BASE32_ALPHABET[index as usize] as char);
        }
    }

    if bits_len > 0 {
        bits = bits << (5 - bits_len);
        let index = bits & 0b11111;
        output.push(BASE32_ALPHABET[index as usize] as char);
        output.push('=');
    }

    while output.len() % 8 != 0 {
        output.push('=');
    }

    output
}

fn base32_decode(input: &str) -> Result<Vec<u8>, Base32Error> {
    let input = input.trim_end_matches('=');
    let mut bits = 0u64;
    let mut bits_len = 0;
    let mut output = Vec::new();

    for c in input.chars() {
        let index = match c {
            'A'..='Z' => c as u8 - b'A',
            '2'..='7' => c as u8 - b'2' + 26,
            _ => return Err(Base32Error::InvalidCharError(c)),
        };
        if index >= 32 {
            return Err(Base32Error::InvalidCharError(c));
        }
        bits = (bits << 5) | (index as u64);
        bits_len += 5;

        while bits_len >= 8 {
            bits_len -= 8;
            let byte = (bits >> bits_len) & 0xFF;
            output.push(byte as u8);
        }
    }

    if bits_len >= 5 {
        return Err(Base32Error::DecodeLengthError);
    }

    println!("{:?}", output);
    Ok(output)
}

/// Implement a base32 convert tool
fn main() -> anyhow::Result<()> {
    let encoded = "JFFEIT2TJFFEMV2KIYSFOSRKIZFCUSSGK5IUSRKKK5FUYRTEMRFQ====";
    match base32_decode(encoded) {
        Ok(decoded) => println!("Decoded: {:?}", decoded),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{base32_decode, base32_encode};

    #[test]
    fn test_base32_encode() {
        let input = "IJDOSIJFWJF$WJ*FJ*JFWQIEJWKLFddK".as_bytes();
        let target =
            String::from_str("JFFEIT2TJFFEMV2KIYSFOSRKIZFCUSSGK5IUSRKKK5FUYRTEMRFQ====").unwrap();
        let result = base32_encode(input);
        assert_eq!(target, result);
    }

    #[test]
    fn test_base32_decode() {
        let input = "JFFEIT2TJFFEMV2KIYSFOSRKIZFCUSSGK5IUSRKKK5FUYRTEMRFQ====";
        let target = "IJDOSIJFWJF$WJ*FJ*JFWQIEJWKLFddK".to_string();
        let result = base32_decode(input).unwrap();
        let result = String::from_utf8_lossy(&result);
        assert_eq!(target, result);
    }
}
