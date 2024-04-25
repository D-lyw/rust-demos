use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub sub: String,
    pub aud: String,
    pub exp: String,
}

pub fn handle_jwt_generate(sub: String, aud: String, exp: String) -> anyhow::Result<Vec<u8>> {
    // TODO: must set format useful exp value here, otherwise error in verify step
    let payload = Payload { sub, aud, exp };

    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    Ok(token.into_bytes())
}

pub fn handle_jwt_verify(token: String) -> anyhow::Result<TokenData<Payload>> {
    let payload: TokenData<Payload> = decode::<Payload>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .unwrap();

    Ok(payload)
}
