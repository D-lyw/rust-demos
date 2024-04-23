use std::io::Read;

use crate::{cli::text::VerifyOpts, text::TextSignFormat, vec_to_array};
use anyhow::{Ok, Result};
use blake3::keyed_hash;
use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey};

pub trait TextSigner {
    fn sign(&self, render: &mut dyn Read) -> Result<Vec<u8>>;
}

struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hasher = keyed_hash(&self.key, &buf);
        Ok(hasher.as_bytes().to_vec())
    }
}

pub struct Ed25519Signer {
    pub key: SigningKey,
}

impl Ed25519Signer {
    fn new(key: [u8; 32]) -> Self {
        let key = SigningKey::from_bytes(&key);
        Self { key }
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // let signature = self.key.sign(&buf);

        // Ok(signature.to_bytes().to_vec())
        Ok(vec![])
    }
}

pub fn handle_text_sign(
    text: &mut dyn Read,
    key: Vec<u8>,
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let mut signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::new(vec_to_array(key))),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::new(vec_to_array(key))),
    };

    signer.sign(text)
}

pub fn handle_text_verify(verify_opts: VerifyOpts) -> Result<(), anyhow::Error> {
    let _ = verify_opts;

    Ok(())
}
