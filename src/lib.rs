#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "std")]
extern crate std;

mod encoding;
mod encryption;
mod error;
mod id;

#[cfg(test)]
mod tests;

pub use aes;

pub use self::{
    error::Error,
    id::{tag, Id},
};

pub type Result<T> = core::result::Result<T, Error>;
