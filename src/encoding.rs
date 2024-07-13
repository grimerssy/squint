static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encode(n: u128) -> impl Iterator<Item = char> {
    let base = ALPHABET.len() as u128;
    let bytes = ALPHABET.as_bytes();
    core::iter::successors(Some(n), move |&n| match n / base {
        0 => None,
        d => Some(d),
    })
    .map(move |i| bytes[(i % base) as usize] as char)
}

pub fn decode(s: &str) -> crate::Result<u128> {
    let base = ALPHABET.len() as u128;
    s.chars()
        .map(|c| ALPHABET.chars().position(|a| c == a))
        .enumerate()
        .try_fold(0, |acc, (i, n)| match n {
            Some(n) => Ok(acc + n as u128 * base.pow(i as u32)),
            None => Err(crate::Error::UnknownCharacter),
        })
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn alphabet_typos() {
        assert_eq!(
            ALPHABET,
            ('a'..='z').chain('A'..='Z').collect::<String>().as_str(),
            "should contain lower and uppercase alphabetic characters"
        )
    }

    #[test]
    fn encode_decode_identity() {
        proptest!(|(n in u128::MIN..)| {
            let encoded = encode(n).collect::<String>();
            let decoded = decode(&encoded);
            prop_assert!(decoded.is_ok());
            prop_assert_eq!(n, decoded.unwrap());
        });
    }
}
