use anyhow::{Ok, Result};
use std::{fs::File, io::Read};

// handle input from stdin or file
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}

// get key content
pub fn get_key_content(key: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(key)?;
    let mut key_content = Vec::new();
    reader.read_to_end(&mut key_content)?;

    Ok(key_content)
}

// convert Vec[u8] to &[u8; 32]
pub fn vec_to_array(vec: Vec<u8>) -> [u8; 32] {
    let mut array: [u8; 32] = [0; 32];
    array.copy_from_slice(&vec[..]);
    array
}
