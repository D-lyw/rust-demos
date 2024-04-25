use base64::prelude::*;
use ring::aead;
use std::io::Read;

use crate::{text::TextSignFormat, vec_to_array};
use anyhow::{Ok, Result};
use blake3::keyed_hash;
use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey};

pub trait TextSigner {
    fn sign(&mut self, render: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifyer {
    fn verify(&self, render: &mut dyn Read, signature: &[u8]) -> Result<bool>;
}

pub trait TextEncrypt {
    fn encrypt(&self, render: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextDecrypt {
    fn decrypt(&self, render: &mut dyn Read) -> Result<Vec<u8>>;
}

struct TextAEAD {
    key: [u8; 32],
    nonce: [u8; 12],
}

impl TextAEAD {
    fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        Self { key, nonce }
    }
}

impl TextEncrypt for TextAEAD {
    fn encrypt(&self, render: &mut dyn Read) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        render.read_to_end(&mut data)?;

        let key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, self.key.as_slice()).unwrap();
        let key = aead::LessSafeKey::new(key);

        key.seal_in_place_append_tag(
            aead::Nonce::assume_unique_for_key(self.nonce),
            aead::Aad::empty(),
            &mut data,
        )
        .unwrap();

        Ok(data)
    }
}

impl TextDecrypt for TextAEAD {
    fn decrypt(&self, render: &mut dyn Read) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        render.read_to_end(&mut data)?;

        let mut data = BASE64_STANDARD.decode(&mut data)?;

        let key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, self.key.as_slice()).unwrap();
        let key = aead::LessSafeKey::new(key);

        key.open_in_place(
            aead::Nonce::assume_unique_for_key(self.nonce),
            aead::Aad::empty(),
            &mut data,
        )
        .unwrap();

        Ok(data)
    }
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
    fn sign(&mut self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hasher = keyed_hash(&self.key, &buf);
        Ok(hasher.as_bytes().to_vec())
    }
}

impl TextVerifyer for Blake3 {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hasher = keyed_hash(&self.key, &buf);
        Ok(signature == hasher.as_bytes())
    }
}

pub struct Ed25519 {
    key: SigningKey,
}

impl Ed25519 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }
}

impl TextSigner for Ed25519 {
    fn sign(&mut self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerifyer for Ed25519 {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = signature.try_into()?;
        let signature = ed25519_dalek::Signature::from_bytes(signature);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

pub fn handle_text_sign(
    text: &mut dyn Read,
    key: Vec<u8>,
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let mut signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::new(vec_to_array(key))),
        TextSignFormat::Ed25519 => Box::new(Ed25519::try_new(key)?),
    };

    signer.sign(text)
}

pub fn handle_text_verify(
    text: &mut dyn Read,
    key: Vec<u8>,
    format: TextSignFormat,
    sig: Vec<u8>,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifyer> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::new(vec_to_array(key))),
        TextSignFormat::Ed25519 => Box::new(Ed25519::try_new(key)?),
    };

    verifier.verify(text, &sig)
}

pub fn handle_text_encrypt(text: &mut dyn Read, key: Vec<u8>, nonce: Vec<u8>) -> Result<Vec<u8>> {
    let key: [u8; 32] = vec_to_array(key);
    let mut _nonce: [u8; 12] = [0; 12];
    _nonce.copy_from_slice(&nonce);
    let aead = TextAEAD::new(key, _nonce);

    aead.encrypt(text)
}

pub fn handle_text_decrypt(text: &mut dyn Read, key: Vec<u8>, nonce: Vec<u8>) -> Result<Vec<u8>> {
    let key: [u8; 32] = vec_to_array(key);
    let mut _nonce: [u8; 12] = [0; 12];
    _nonce.copy_from_slice(&nonce);
    let aead = TextAEAD::new(key, _nonce);

    aead.decrypt(text)
}
