use aes::{
    cipher::{BlockDecrypt, BlockEncrypt},
    Aes128,
};

pub fn encrypt(tag: u64, id: i64, cipher: &Aes128) -> u128 {
    let tagged = concat(tag, id);
    let mut bytes = tagged.to_le_bytes().into();
    cipher.encrypt_block(&mut bytes);
    u128::from_le_bytes(bytes.into())
}

pub fn decrypt(expected_tag: u64, id: u128, cipher: &Aes128) -> crate::Result<i64> {
    let mut bytes = id.to_le_bytes().into();
    cipher.decrypt_block(&mut bytes);
    let tagged = u128::from_le_bytes(bytes.into());
    match bisect(tagged) {
        (tag, id) if tag == expected_tag => Ok(id),
        _ => Err(crate::Error::WrongTag),
    }
}

fn concat(tag: u64, id: i64) -> u128 {
    let tag = (tag as u128).reverse_bits();
    let id = u64::from_le_bytes(id.to_le_bytes()) as u128;
    tag | id
}

fn bisect(tagged: u128) -> (u64, i64) {
    const HIGH_BITS: u128 = !0 << (u128::BITS / 2);
    const LOW_BITS: u128 = !0 >> (u128::BITS / 2);
    let tag = (tagged & HIGH_BITS).reverse_bits() as u64;
    let id = (tagged & LOW_BITS) as u64;
    let id = i64::from_le_bytes(id.to_le_bytes());
    (tag, id)
}

#[cfg(test)]
mod tests {
    use aes::cipher::KeyInit;
    use proptest::{prelude::*, property_test};

    use super::*;

    #[property_test]
    fn concat_is_deterministic(tag: u64, id: i64) {
        let tagged_bytes = concat(tag, id);
        prop_assert_eq!((tag, id), bisect(tagged_bytes));
    }

    #[property_test]
    fn bisect_is_deterministic(tagged: u128) {
        let (tag, id) = bisect(tagged);
        prop_assert_eq!(tagged, concat(tag, id));
    }

    #[property_test]
    fn encrypt_is_deterministic(tag: u64, id: i64, key: [u8; 16]) {
        let cipher = Aes128::new(&key.into());
        let encrypted = encrypt(tag, id, &cipher);
        let decrypted = decrypt(tag, encrypted, &cipher);
        prop_assert!(decrypted.is_ok_and(|ok| ok == id));
    }

    #[property_test]
    fn decrypt_is_partial(tag: u64, id: u128, key: [u8; 16]) {
        let cipher = Aes128::new(&key.into());
        let decrypted = decrypt(tag, id, &cipher);
        let encrypted = decrypted.map(|id| encrypt(tag, id, &cipher));
        prop_assert!(encrypted.is_err() || encrypted.is_ok_and(|ok| ok == id));
    }
}
