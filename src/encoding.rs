use core::{fmt, iter};

use crate::error::Error;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encode(n: u128, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let base = ALPHABET.len() as u128;
    let bytes = ALPHABET.as_bytes();
    iter::successors(Some(n), |&n| match n / base {
        0 => None,
        d => Some(d),
    })
    .map(|i| bytes[(i % base) as usize] as char)
    .try_for_each(|c| write!(f, "{c}"))
}

pub fn decode(s: &str) -> Result<u128, Error> {
    let base = ALPHABET.len() as u128;
    s.chars()
        .map(|c| ALPHABET.chars().position(|a| c == a))
        .enumerate()
        .try_fold(0, |acc, (i, n)| match n {
            Some(n) => Ok(acc + n as u128 * base.pow(i as u32)),
            None => Err(Error::UnknownCharacter),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet_typos() {
        assert_eq!(
            ALPHABET,
            ('a'..='z').chain('A'..='Z').collect::<String>().as_str(),
            "should contain lower and uppercase alphabetic characters"
        )
    }
}
