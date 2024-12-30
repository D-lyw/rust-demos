use byteorder::{BigEndian, ByteOrder};
use hmac::{Hmac, Mac};
use sha1::Sha1;

fn generate_totp(secret: &str) -> u32 {
    // Decode secret key
    let secret = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret).unwrap();
    // Get current Unix time (seconds since the epoch)
    let current_time = time::OffsetDateTime::now_utc().unix_timestamp();

    let time_interval = (current_time / 30) as u64;

    // Convert to byte array
    let mut time_bytes = [0u8; 8];
    BigEndian::write_u64(&mut time_bytes, time_interval);

    let mut mac = Hmac::<Sha1>::new_from_slice(&secret).unwrap();
    mac.update(&time_bytes);

    let result = mac.finalize().into_bytes();
    let offset = (result[19] & 0xf) as usize;
    println!("offset: {}", offset);
    let code = ((result[offset] & 0x7f) as u32) << 24
        | ((result[offset + 1] & 0xff) as u32) << 16
        | ((result[offset + 2] & 0xff) as u32) << 8
        | (result[offset + 3] & 0xff) as u32;

    println!("code: {}", code);
    code % 1_000_000
}

// Implement the TOTP algorithm
fn main() {
    let secret_key = "JBSWY3DPEHPK3PXP"; // Base32 encoded
    let code = generate_totp(secret_key);
    println!("TOTP code: {}", code);
}
