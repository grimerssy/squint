#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "std")]
extern crate std;

mod encoding;
mod encryption;
mod error;
mod id;

#[cfg(feature = "tag")]
mod tag;

pub use aes;

pub use error::Error;
pub use id::Id;

#[cfg(feature = "tag")]
pub use tag::tag;

pub type Result<T> = core::result::Result<T, Error>;
