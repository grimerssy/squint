use core::iter;

use crate::Error;

const ENCODING_LENGTH: usize = MAX_UNPADDED_LEN + PADDING_SIZE_LEN;

const MAX_UNPADDED_LEN: usize = 22;

const PADDING_SIZE_LEN: usize = 1;

static ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

static RADIX: u128 = ALPHABET.len() as u128;

pub fn encode(n: u128) -> [char; ENCODING_LENGTH] {
    let mut digits = <[char; ENCODING_LENGTH]>::default();
    let mut writer = digits.iter_mut();
    let unpadded = digits_of(n)
        .zip(writer.by_ref())
        .map(|(digit, w)| *w = digit)
        .count();
    gen_padding(n, ENCODING_LENGTH - unpadded)
        .zip(writer)
        .for_each(|(digit, w)| *w = digit);
    digits
}

pub fn decode(digits: &str) -> crate::Result<u128> {
    if digits.chars().count() != ENCODING_LENGTH {
        return Err(Error::InvalidLength);
    }
    let mut digits = digits.chars();
    let padding_size = digits.by_ref().rev().take(PADDING_SIZE_LEN);
    let padding_size = match parse_number(padding_size) {
        Ok(0) | Err(_) => Err(Error::WrongPadding),
        Ok(size) => Ok(size as usize),
    }?;
    let encoding_size = ENCODING_LENGTH
        .checked_sub(padding_size)
        .unwrap_or_default();
    let n = parse_number(digits.by_ref().take(encoding_size))?;
    let filler_padding = gen_padding(n, padding_size).take(padding_size - PADDING_SIZE_LEN);
    if digits.eq(filler_padding) {
        Ok(n)
    } else {
        Err(Error::WrongPadding)
    }
}

fn digits_of(n: u128) -> impl Iterator<Item = char> {
    iter::successors(Some(n), move |&n| match n / RADIX {
        0 => None,
        n => Some(n),
    })
    .map(|x| x % RADIX)
    .map(to_digit)
}

fn to_digit(digit: u128) -> char {
    ALPHABET.as_bytes()[digit as usize] as char
}

fn gen_padding(seed: u128, size: usize) -> impl Iterator<Item = char> {
    iter::successors(Some(seed.max(u128::MAX - seed)), |n| Some(n ^ (n << 1)))
        .flat_map(digits_of)
        .take(size - PADDING_SIZE_LEN)
        .chain(encode_padding_size(size))
}

fn encode_padding_size(size: usize) -> [char; PADDING_SIZE_LEN] {
    let mut digits = [to_digit(0); PADDING_SIZE_LEN];
    let encoded_size = || digits_of(size as u128);
    let leading_zeroes = PADDING_SIZE_LEN - encoded_size().count();
    digits
        .iter_mut()
        .skip(leading_zeroes)
        .zip(encoded_size())
        .for_each(|(w, digit)| *w = digit);
    digits
}

fn parse_number(digits: impl Iterator<Item = char>) -> crate::Result<u128> {
    digits
        .enumerate()
        .map(|(i, digit)| {
            let digit = parse_digit(digit)?;
            RADIX
                .checked_pow(i as u32)
                .and_then(|value| value.checked_mul(digit))
                .ok_or(Error::Overflow)
        })
        .try_fold(0, |acc, n| {
            n.and_then(|n| n.checked_add(acc).ok_or(Error::Overflow))
        })
}

fn parse_digit(digit: char) -> crate::Result<u128> {
    ALPHABET
        .chars()
        .position(|x| x == digit)
        .map(|i| i as u128)
        .ok_or(Error::UnknownCharacter)
}

#[cfg(test)]
mod tests {
    use proptest::{prelude::*, property_test, sample::SizeRange};

    use super::*;

    #[test]
    fn length_assumptions_are_met() {
        assert_eq!(MAX_UNPADDED_LEN, digits_of(u128::MAX).count());
        let max_padding_size = ENCODING_LENGTH - digits_of(u128::MIN).count();
        assert_eq!(
            PADDING_SIZE_LEN,
            digits_of(max_padding_size as u128).count()
        );
    }

    #[test]
    fn alphabet_is_base58() {
        let blacklist = ['I', 'O', 'l', '0'];
        let base58 = ('0'..='9')
            .chain('A'..='Z')
            .chain('a'..='z')
            .filter(|c| !blacklist.contains(c))
            .collect::<String>();
        assert_eq!(ALPHABET, &base58);
    }

    #[property_test]
    fn encode_is_deterministic(n: u128) {
        let encoded = encode_str(n);
        let decoded = decode(&encoded);
        prop_assert!(decoded.is_ok());
        prop_assert_eq!(n, decoded.unwrap());
    }

    #[test]
    fn decode_is_partial() {
        let check = |s: &str| {
            let decoded = decode(s);
            decoded.is_err() || decoded.is_ok_and(|ok| encode_str(ok) == s)
        };
        proptest!(|(s in any::<String>())| prop_assert!(check(&s)));
        proptest!(|(s in sanitized_string())| prop_assert!(check(&s)));
    }

    fn encode_str(n: u128) -> String {
        encode(n).into_iter().collect()
    }

    fn sanitized_string() -> impl Strategy<Value = String> {
        prop::collection::vec(0..RADIX, SizeRange::default())
            .prop_map(|digits| digits.into_iter().map(to_digit).collect())
    }
}
