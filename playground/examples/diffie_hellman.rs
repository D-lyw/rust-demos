use aes::cipher::generic_array::GenericArray;
use aes::cipher::BlockDecrypt;
use aes::cipher::BlockEncrypt;
use aes::cipher::KeyInit;
use aes::Aes128;
use aes::Block;
use rand_core::OsRng;

use ristretto255_dh::EphemeralSecret;
use ristretto255_dh::PublicKey;

use sha2::Sha256;

fn main() {
    // Alice's side
    let alice_secret = EphemeralSecret::new(&mut OsRng);
    let alice_public = PublicKey::from(&alice_secret);

    // Bob's side
    let bob_secret = EphemeralSecret::new(&mut OsRng);
    let bob_public = PublicKey::from(&bob_secret);

    // Alice again
    let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);

    // Bob again
    let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);

    let alice_shared_secret_bytes = <[u8; 32]>::from(alice_shared_secret);
    let bob_shared_secret_bytes = <[u8; 32]>::from(bob_shared_secret);

    // Each peer's computed shared secret should be the same.
    assert_eq!(alice_shared_secret_bytes, bob_shared_secret_bytes);

    let cipher = generate_aes_key_cipher(&alice_shared_secret_bytes);

    let mut block = Block::clone_from_slice(b"Hello, world!kkk");
    cipher.encrypt_block(&mut block);
    println!("Ciphertext: {:?}", block);
    
    let mut decrypted_block = Block::clone_from_slice(&block);
    cipher.decrypt_block(&mut decrypted_block);
    println!("Decrypted block: {:?}", String::from_utf8(decrypted_block.to_vec()).unwrap());
}

fn generate_aes_key_cipher(shared_secret: &[u8; 32]) -> Aes128 {
    let hkdf = hkdf::Hkdf::<Sha256>::new(None, shared_secret);
    let mut aes_key = [0u8; 16];
    hkdf.expand(b"aes-128-key", &mut aes_key).unwrap();

    Aes128::new(GenericArray::from_slice(&aes_key))
}
