use crate::Error;

static ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

static RADIX: u128 = ALPHABET.len() as u128;

const TARGET_LENGTH: usize = 23;

pub fn encode(n: u128) -> [char; TARGET_LENGTH] {
    let mut digits = <[char; TARGET_LENGTH]>::default();
    let mut writer = digits.iter_mut();
    let unpadded = digits_of(n)
        .zip(writer.by_ref())
        .map(|(digit, dest)| *dest = digit)
        .count();
    let padding_len = TARGET_LENGTH - unpadded;
    digits_of(u128::MAX - n)
        .take(padding_len - 1)
        .chain([to_digit(padding_len as u128)])
        .zip(writer)
        .for_each(|(digit, dest)| *dest = digit);
    digits
}

pub fn decode(digits: &str) -> crate::Result<u128> {
    let padding = digits
        .chars()
        .next_back()
        .ok_or(Error::NoPadding)
        .and_then(parse_digit)
        .and_then(|padding| match padding {
            0 => Err(Error::NoPadding),
            p => Ok(p as usize),
        })?;
    let mut digits = digits.chars();
    digits.by_ref().rev().take(padding).last();
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
            n.and_then(|n| n.checked_add(acc).ok_or(crate::Error::Overflow))
        })
}

fn digits_of(n: u128) -> impl Iterator<Item = char> {
    core::iter::successors(Some(n), move |&prev| match prev / RADIX {
        0 => None,
        next => Some(next),
    })
    .map(|x| x % RADIX)
    .map(to_digit)
}

fn to_digit(digit: u128) -> char {
    ALPHABET.as_bytes()[digit as usize] as char
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
    fn alphabet_typos() {
        let blacklist = ['I', 'O', 'l', '0'];
        let base58 = ('0'..='9')
            .chain('A'..='Z')
            .chain('a'..='z')
            .filter(|c| !blacklist.contains(c))
            .collect::<String>();
        assert_eq!(ALPHABET, &base58, "should contain valid base58 charset");
    }

    #[test]
    fn check_target_length() {
        assert_eq!(TARGET_LENGTH, digits_of(u128::MAX).count() + 1);
    }

    #[property_test]
    fn encode_decode_identity(n: u128) {
        let encoded = encode_str(n);
        let decoded = decode(&encoded);
        prop_assert!(decoded.is_ok());
        prop_assert_eq!(n, decoded.unwrap());
    }

    #[property_test]
    fn decode_unsanitized(s: String) {
        decode(&s).ok();
    }

    #[test]
    fn decode_sanitized() {
        let sanitized_string = prop::collection::vec(0..RADIX, SizeRange::default())
            .prop_map(|digits| digits.into_iter().map(to_digit).collect::<String>());
        proptest!(|(s in sanitized_string)| {
            decode(&s).ok();
        });
    }

    fn encode_str(n: u128) -> String {
        encode(n).into_iter().collect()
    }
}
