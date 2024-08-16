use aes::{cipher::KeyInit, Aes128};
use prop_test::prelude::*;
use squint::Id;

pub fn any_cipher() -> impl Strategy<Value = Aes128> {
    any::<[u8; 16]>().prop_map(|key| Aes128::new(&key.into()))
}

fn id_to_string_and_back<const TAG: u64>(id: i64, cipher: &Aes128) -> squint::Result<i64> {
    Id::<TAG>::new(id, cipher)
        .to_string()
        .parse()
        .and_then(|id: Id<TAG>| id.to_raw(cipher))
}

#[test]
fn id_decodes_back() {
    prop_test!(&(any::<i64>(), any_cipher()), |(id, cipher)| {
        let parsed = id_to_string_and_back::<0>(id, &cipher);
        prop_assert!(parsed.is_ok());
        prop_assert_eq!(id, parsed.unwrap());
        Ok(())
    });
}
