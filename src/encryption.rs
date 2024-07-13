use aes::{
    cipher::{BlockDecrypt, BlockEncrypt},
    Aes128,
};

pub fn encrypt(tag: u64, id: i64, cipher: &Aes128) -> u128 {
    let tagged = concat(tag, id);
    let mut bytes = tagged.into();
    cipher.encrypt_block(&mut bytes);
    u128::from_le_bytes(bytes.into())
}

pub fn decrypt(expected_tag: u64, id: u128, cipher: &Aes128) -> crate::Result<i64> {
    let mut bytes = id.to_le_bytes().into();
    cipher.decrypt_block(&mut bytes);
    match bisect(bytes.into()) {
        (tag, id) if tag == expected_tag => Ok(id),
        _ => Err(crate::Error::WrongTag),
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

#[cfg(test)]
mod tests {
    use aes::cipher::KeyInit;
    use proptest::{array, prelude::*};

    use super::*;

    proptest! {
        #[test]
        fn concat_bisect_identity(
            tag in u64::MIN..u64::MAX,
            id in 1_i64..i64::MAX
        ) {
            let tagged_bytes = concat(tag, id);
            let (extracted_tag, extracted_id) = bisect(tagged_bytes);
            prop_assert_eq!(tag, extracted_tag);
            prop_assert_eq!(id, extracted_id);
        }

        #[test]
        fn encrypt_decrypt_identity(
            tag in u64::MIN..u64::MAX,
            id in 1_i64..i64::MAX,
            key in array::uniform16(0_u8..)
        ) {
            let cipher = Aes128::new(&key.into());
            let encrypted = encrypt(tag, id, &cipher);
            let decrypted = decrypt(tag, encrypted, &cipher);
            prop_assert!(decrypted.is_ok());
            prop_assert_eq!(decrypted.unwrap(), id);
        }
    }
}
