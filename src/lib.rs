#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

mod encoding;
mod encryption;
mod error;

use core::{fmt, str::FromStr};

use aes::Aes128;

use self::{
    encoding::{decode, encode},
    encryption::{decrypt, encrypt},
    error::Error,
};

#[derive(Clone, Copy)]
pub struct Id<const TAG: u64 = 0>(u128);

pub const fn tag(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut result = 0u64;
    let mut i = 0;
    while i < bytes.len() {
        result |= (bytes[i] as u64) << (8 * i);
        i += 1;
    }
    result
}

impl<const TAG: u64> Id<TAG> {
    pub fn new(id: i64, cipher: &Aes128) -> Self {
        Self(encrypt(TAG, id, cipher))
    }

    pub fn to_raw(self, cipher: &Aes128) -> Result<i64, Error> {
        decrypt(TAG, self.0, cipher)
    }
}

impl<const TAG: u64> fmt::Display for Id<TAG> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        encode(self.0, f)
    }
}

impl<const TAG: u64> fmt::Debug for Id<TAG> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&format_args!("{self}")).finish()
    }
}

impl<const TAG: u64> FromStr for Id<TAG> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        decode(s).map(Self)
    }
}
