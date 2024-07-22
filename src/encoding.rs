use crate::Error;

static ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

static RADIX: u128 = ALPHABET.len() as u128;

pub fn encode(n: u128) -> impl Iterator<Item = char> {
    core::iter::successors(Some(n), move |&prev| match prev / RADIX {
        0 => None,
        next => Some(next),
    })
    .map(|x| x % RADIX)
    .map(to_digit)
}

pub fn decode(digits: impl Iterator<Item = char>) -> crate::Result<u128> {
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

    #[property_test]
    fn encode_decode_identity(n: u128) {
        let encoded = encode(n);
        let decoded = decode(encoded);
        prop_assert!(decoded.is_ok());
        prop_assert_eq!(n, decoded.unwrap());
    }

    #[property_test]
    fn decode_unsanitized(s: String) {
        decode(s.chars()).ok();
    }

    #[test]
    fn decode_sanitized() {
        let sanitized_string = prop::collection::vec(0..RADIX, SizeRange::default())
            .prop_map(|digits| digits.into_iter().map(to_digit).collect::<String>());
        proptest!(|(s in sanitized_string)| {
            decode(s.chars()).ok();
        });
    }
}
