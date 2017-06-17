//! A clever way to find anagrams
//!
//! > Map each of the 26 English charaters A, B, C, D, ... [for simplicity] to a
//! > prime number. Then multiply the characters of each word. Since every
//! > integer is a prime or a *unique* product of primes (fundamental theorem of
//! > arithmetic), two words are anagrams if their products are the same.
//! >
//! > -- From <https://twitter.com/fermatslibrary/status/875340896379817984>
//!
//! # Examples
//!
//! ```rust
//! extern crate is_anagram;
//! use is_anagram::IsAnagram;
//!
//! # fn main() {
//! assert!("banana".is_anagram_of("nabana").unwrap());
//! assert!(!"lorem".is_anagram_of("ipsum").unwrap());
//! # }
//! ```

extern crate ascii;
#[macro_use]
extern crate error_chain;

use std::ascii::AsciiExt;
use ascii::{AsAsciiStr, AsciiStr, AsciiChar};

pub(crate) use result::*;
pub use result::Error;

static PRIMES: [u64; 26] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101,
];

fn anagram_to_prime_product(input: &AsciiStr) -> u64 {
    input
        .chars()
        .map(AsciiChar::to_ascii_lowercase)
        .map(|c| (c as u8) - b'a')
        .map(|idx| PRIMES[idx as usize])
        .product()
}

pub trait IsAnagram {
    fn is_anagram_of(&self, rhs: &str) -> Result<bool>;
}

impl<'a> IsAnagram for &'a str {
    fn is_anagram_of(&self, rhs: &str) -> Result<bool> {
        ensure!(self.len() == rhs.len(), ErrorKind::DifferentLength);
        ensure!(!self.is_empty(), ErrorKind::NotOneWord);
        ensure!(!rhs.is_empty(), ErrorKind::NotOneWord);
        ensure!(!self.contains(' '), ErrorKind::NotOneWord);
        ensure!(!rhs.contains(' '), ErrorKind::NotOneWord);

        let lhs_product = anagram_to_prime_product(self.as_ascii_str()?);
        let rhs_product = anagram_to_prime_product(rhs.as_ascii_str()?);
        Ok(lhs_product == rhs_product)
    }
}

mod result {
    error_chain! {
        foreign_links {
            Ascii(::ascii::AsAsciiStrError);
        }

        errors {
            DifferentLength {
                description("Words have different lengths")
            }
            NotOneWord {
                description("Input is not a single word")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsAnagram;

    #[test]
    fn postive() {
        assert!("banana".is_anagram_of("nabana").unwrap());
    }

    #[test]
    fn negative() {
        assert!(!"banana".is_anagram_of("pineap").unwrap());
        assert!(!"lorem".is_anagram_of("ipsum").unwrap());
    }

    macro_rules! assert_err {
        ($got:expr, $expected:pat) => {{
            assert!($got.is_err(), "Expected error, got {:?}", $got);
            let err = $got.unwrap_err();
            let kind = err.kind();

            if let $expected = *kind {
                println!("Found `{}` as expected", stringify!($expected));
            } else {
                panic!("Got `{:?}`, but expected type `{}`", kind, stringify!($expected));
            }
        }}
    }

    #[test]
    fn errors() {
        use ::result::ErrorKind;

        assert_err!("a".is_anagram_of(""), ErrorKind::DifferentLength);
        assert_err!("".is_anagram_of("a"), ErrorKind::DifferentLength);

        assert_err!(
            "banana".is_anagram_of("pineapple"),
            ErrorKind::DifferentLength
        );

        assert_err!("foo bar".is_anagram_of("bar foo"), ErrorKind::NotOneWord);
    }
}
