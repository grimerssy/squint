use core::iter;

const MAX_UNPADDED_LEN: usize = 22;

const PADDING_SIZE_LEN: usize = 1;

const ENCODING_LENGTH: usize = MAX_UNPADDED_LEN + PADDING_SIZE_LEN;

static ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

static RADIX: u128 = ALPHABET.len() as u128;

pub fn stringify(n: u128) -> impl Iterator<Item = char> {
    let mut buffer = <[u8; ENCODING_LENGTH]>::default();
    let mut writer = buffer.iter_mut();
    let unpadded = digits_of(n)
        .zip(writer.by_ref())
        .map(|(digit, w)| *w = digit)
        .count();
    gen_padding(n, ENCODING_LENGTH - unpadded)
        .zip(writer)
        .for_each(|(digit, w)| *w = digit);
    buffer.into_iter().map(char::from)
}

pub fn parse(digits: &str) -> Option<u128> {
    let mut digits = match digits {
        s if s.len() != ENCODING_LENGTH => None,
        s => Some(s.bytes()),
    }?;
    let padding_size = digits.by_ref().rev().take(PADDING_SIZE_LEN);
    let padding_size = match parse_number(padding_size) {
        Some(0) | None => None,
        Some(size) => Some(size as usize),
    }?;
    let encoding_size = ENCODING_LENGTH
        .checked_sub(padding_size)
        .unwrap_or_default();
    let n = parse_number(digits.by_ref().take(encoding_size))?;
    let filler_padding = gen_padding(n, padding_size).take(padding_size - PADDING_SIZE_LEN);
    if digits.eq(filler_padding) {
        Some(n)
    } else {
        None
    }
}

fn digits_of(n: u128) -> impl Iterator<Item = u8> {
    iter::successors(Some(n), move |&n| match n / RADIX {
        0 => None,
        n => Some(n),
    })
    .map(last_digit_of)
}

fn last_digit_of(n: u128) -> u8 {
    let digit = n % RADIX;
    ALPHABET[digit as usize]
}

fn gen_padding(seed: u128, size: usize) -> impl Iterator<Item = u8> {
    iter::successors(Some(seed.max(u128::MAX - seed)), |n| Some(n ^ (n << 1)))
        .flat_map(digits_of)
        .take(size - PADDING_SIZE_LEN)
        .chain(encode_padding_size(size as u128))
}

fn encode_padding_size(size: u128) -> [u8; PADDING_SIZE_LEN] {
    let mut digits = [last_digit_of(0); PADDING_SIZE_LEN];
    let leading_zeroes = PADDING_SIZE_LEN - digits_of(size).count();
    digits
        .iter_mut()
        .skip(leading_zeroes)
        .zip(digits_of(size))
        .for_each(|(w, digit)| *w = digit);
    digits
}

fn parse_number(digits: impl Iterator<Item = u8>) -> Option<u128> {
    digits
        .enumerate()
        .map(|(i, digit)| {
            let digit = parse_digit(digit)?;
            RADIX
                .checked_pow(i as u32)
                .and_then(|value| value.checked_mul(digit))
        })
        .try_fold(0, |acc, n| n.and_then(|n| n.checked_add(acc)))
}

fn parse_digit(digit: u8) -> Option<u128> {
    let digit = match digit {
        b'1'..=b'9' => digit - b'1',
        b'A'..=b'H' => digit - b'A' + 9,
        b'J'..=b'N' => digit - b'J' + 17,
        b'P'..=b'Z' => digit - b'P' + 22,
        b'a'..=b'k' => digit - b'a' + 33,
        b'm'..=b'z' => digit - b'm' + 44,
        _ => return None,
    };
    Some(digit as u128)
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::tests::prop_test;

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
        assert_eq!(ALPHABET, base58.as_bytes());
    }

    #[test]
    fn digits_parse_back() {
        let digits = (0..RADIX).collect::<Vec<_>>();
        let parsed = digits
            .iter()
            .copied()
            .map(last_digit_of)
            .map(parse_digit)
            .collect::<Option<Vec<_>>>();
        assert!(parsed.is_some());
        assert_eq!(digits, parsed.unwrap());
    }

    #[test]
    fn parse_last_digit_gives_mod_radix() {
        prop_test!(&any::<u128>(), |n| {
            let ascii_digit = last_digit_of(n);
            let parsed_digit = parse_digit(ascii_digit);
            prop_assert!(parsed_digit.is_some());
            prop_assert_eq!(n % RADIX, parsed_digit.unwrap());
            Ok(())
        });
    }

    #[test]
    fn block_decodes_back() {
        prop_test!(&any::<u128>(), |n| {
            let encoded = stringify(n);
            let decoded = parse(&encoded);
            prop_assert!(decoded.is_some());
            prop_assert_eq!(n, decoded.unwrap());
            Ok(())
        });
    }

    fn stringify(n: u128) -> String {
        super::stringify(n).collect()
    }
}
