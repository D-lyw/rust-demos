use std::{thread::sleep, time::Duration};

use totp_rs::{Secret, TOTP};

/// Example of using totp_rs ceate
/// 
/// Resource:
/// + check onlie https://totp.danhersam.com/
/// + HOTP design docs: https://datatracker.ietf.org/doc/html/rfc4226
fn main() -> anyhow::Result<()> {
    let secret =
        Secret::Encoded("JBSWY3DPEHPK3PXPJDKSLJKLDSJJIOJWEIJIOJDIJDLSJLKJDSKJ".to_string());
    let totp = TOTP::new(totp_rs::Algorithm::SHA1, 6, 1, 30, secret.to_bytes()?)?;

    let mut count = 0;
    loop {
        count += 1;
        let token = totp.generate_current()?;
        println!("Current TOTP token: {}, {}", token, totp);
        println!("TTL: {}", totp.ttl()?);

        sleep(Duration::from_secs(30));
        if count > 10 {
            break;
        }
    }

    Ok(())
}
