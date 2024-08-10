use aes::{
    cipher::{BlockDecrypt, BlockEncrypt},
    Aes128,
};

pub fn encrypt(tag: u64, id: i64, cipher: &Aes128) -> [u8; 16] {
    let mut block = concat(tag, id).into();
    cipher.encrypt_block(&mut block);
    block.into()
}

pub fn decrypt(block: [u8; 16], cipher: &Aes128) -> (u64, i64) {
    let mut block = block.into();
    cipher.decrypt_block(&mut block);
    bisect(block.into())
}

fn concat(tag: u64, id: i64) -> [u8; 16] {
    let mut block = <[u8; 16]>::default();
    let tag = tag.reverse_bits(); // TODO remove in next breaking release
    id.to_ne_bytes()
        .into_iter()
        .chain(tag.to_le_bytes())
        .zip(block.iter_mut())
        .for_each(|(byte, w)| *w = byte);
    block
}

fn bisect(block: [u8; 16]) -> (u64, i64) {
    let mut tag = <[u8; 8]>::default();
    let mut id = <[u8; 8]>::default();
    id.iter_mut()
        .chain(tag.iter_mut())
        .zip(block)
        .for_each(|(w, byte)| *w = byte);
    let (tag, id) = (u64::from_le_bytes(tag), i64::from_le_bytes(id));
    let tag = tag.reverse_bits(); // TODO remove in next breaking release
    (tag, id)
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::tests::{any_cipher, prop_test};

    use super::*;

    #[test]
    fn concat_bisect_is_identity() {
        let concat_bisect = |(tag, id)| {
            let block = concat(tag, id);
            bisect(block)
        };
        prop_test!(&any::<(u64, i64)>(), |x| {
            prop_assert_eq!(x, concat_bisect(x));
            Ok(())
        });
    }

    #[test]
    fn encrypt_decrypt_is_identity() {
        let encrypt_decrypt = |(tag, id), cipher| {
            let block = encrypt(tag, id, &cipher);
            decrypt(block, &cipher)
        };
        prop_test!(&(any::<(u64, i64)>(), any_cipher()), |(x, cipher)| {
            prop_assert_eq!(x, encrypt_decrypt(x, cipher));
            Ok(())
        });
    }
}
