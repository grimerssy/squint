use core::{
    fmt::{self, Write},
    str::FromStr,
};

use aes::Aes128;

use crate::{
    encoding::{parse, stringify},
    encryption::{decrypt, encrypt},
    error::Error,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Id<const TAG: u64 = 0>(u128);

pub const fn tag(tag: &str) -> u64 {
    assert!(tag.len() <= 8, "tag is too long");
    let mut byte_block = [0; 8];
    let tag = tag.as_bytes();
    let mut i = 0;
    while i < tag.len() {
        byte_block[i] = tag[i];
        i += 1;
    }
    u64::from_be_bytes(byte_block)
}

impl<const TAG: u64> Id<TAG> {
    pub fn new(id: i64, cipher: &Aes128) -> Self {
        Self(encrypt(TAG, id, cipher))
    }

    pub fn to_raw(self, cipher: &Aes128) -> crate::Result<i64> {
        match decrypt(self.0, cipher) {
            (tag, id) if tag == TAG => Ok(id),
            _ => Err(Error::TagMismatch),
        }
    }
}

impl<const TAG: u64> fmt::Display for Id<TAG> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        stringify(self.0).try_for_each(|c| f.write_char(c))
    }
}

impl<const TAG: u64> fmt::Debug for Id<TAG> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&format_args!("{self}")).finish()
    }
}

impl<const TAG: u64> FromStr for Id<TAG> {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        parse(s).map(Self).ok_or(Error::InvalidFormat)
    }
}
