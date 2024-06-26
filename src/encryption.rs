use aes::{
    cipher::{BlockDecrypt, BlockEncrypt},
    Aes128,
};

use crate::error::Error;

pub fn encrypt(tag: u64, id: i64, cipher: &Aes128) -> u128 {
    let tagged = concat(tag, id);
    let mut bytes = tagged.into();
    cipher.encrypt_block(&mut bytes);
    u128::from_le_bytes(bytes.into())
}

pub fn decrypt(expected_tag: u64, id: u128, cipher: &Aes128) -> Result<i64, Error> {
    let mut bytes = id.to_le_bytes().into();
    cipher.decrypt_block(&mut bytes);
    match bisect(bytes.into()) {
        (tag, id) if tag == expected_tag => Ok(id),
        _ => Err(Error::WrongTag),
    }
}

fn concat(tag: u64, id: i64) -> [u8; 16] {
    let tag = (tag as u128).reverse_bits();
    let id = u64::from_le_bytes(id.to_le_bytes()) as u128;
    (tag | id).to_le_bytes()
}

fn bisect(bytes: [u8; 16]) -> (u64, i64) {
    const HIGH_BITS: u128 = !0 << (u128::BITS / 2);
    const LOW_BITS: u128 = !0 >> (u128::BITS / 2);
    let tagged = u128::from_le_bytes(bytes);
    let tag = (tagged & HIGH_BITS).reverse_bits() as u64;
    let id = (tagged & LOW_BITS) as u64;
    let id = i64::from_le_bytes(id.to_le_bytes());
    (tag, id)
}
