use aes::{
    cipher::{BlockDecrypt, BlockEncrypt},
    Aes128,
};

pub fn encrypt(tag: u64, id: i64, cipher: &Aes128) -> u128 {
    let mut block = concat(tag, id).into();
    cipher.encrypt_block(&mut block);
    u128::from_le_bytes(block.into())
}

pub fn decrypt(n: u128, cipher: &Aes128) -> (u64, i64) {
    let mut block = n.to_le_bytes().into();
    cipher.decrypt_block(&mut block);
    bisect(block.into())
}

fn concat(tag: u64, id: i64) -> [u8; 16] {
    let mut block = <[u8; 16]>::default();
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
    (u64::from_le_bytes(tag), i64::from_le_bytes(id))
}

#[cfg(test)]
mod tests {
    use core::{fmt::Binary, mem::size_of};

    use aes::cipher::KeyInit;
    use prop_test::prelude::*;

    use super::*;

    #[test]
    fn binary_of_concat_is_concat_of_binaries() {
        fn padded_binary<T: Binary>(x: T) -> String {
            format!("{x:0size$b}", size = size_of::<T>() * 8)
        }
        prop_test!(&any::<(u64, i64)>(), |(tag, id)| {
            let concatenated_binaries = [padded_binary(tag), padded_binary(id)].concat();
            let block = concat(tag, id);
            let block_binary = padded_binary(u128::from_le_bytes(block));
            prop_assert_eq!(concatenated_binaries, block_binary);
            Ok(())
        });
    }

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

    pub fn any_cipher() -> impl Strategy<Value = Aes128> {
        any::<[u8; 16]>().prop_map(|key| Aes128::new(&key.into()))
    }
}
