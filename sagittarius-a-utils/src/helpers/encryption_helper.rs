use std::{
    env
};

use data_encoding::HEXUPPER;

use ring::error::{self};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

fn get_salt() -> [u8; 64] {
    let bytes_to_salt = env::var("ENCRYPT_SALT").unwrap_or_default().to_string();
    let byte4: [u8;64] = bytes_to_salt.as_bytes().try_into().expect("failed to load");
    byte4
}


pub fn encrypt(password: String) -> Result<String, error::Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(901).unwrap();

    let salt = get_salt();

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    let pass = HEXUPPER.encode(&pbkdf2_hash);

    return Ok(pass);
}
