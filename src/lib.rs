//! Textual representation of base 95 fractional numbers with arbitrary precision,
//! intended to be used in real-time collaborative applications.
//!
//! It can only represent numbers between 0 and 1, exclusive.
//! The leading `0.` is omitted.
//!
//! Heavily inspired by [this article](https://www.figma.com/blog/realtime-editing-of-ordered-sequences/).
//!
//! ## Why 95?
//!
//! - UTF-8, the most popular Unicode encoding scheme, can encode ASCII as is. (1 byte per character)
//! - ASCII has 95 printable characters in total, from space to tilde.
//!
//! ## Example
//!
//! ```
//! use base95::Base95;
//! use std::str::FromStr;
//!
//! let n1 = Base95::mid();
//! assert_eq!(n1.to_string(), "O");
//! assert_eq!(n1.raw_digits(), vec![47]);
//!
//! let n2 = Base95::avg_with_zero(&n1);
//! assert_eq!(n2.to_string(), "7");
//! assert_eq!(n2.raw_digits(), vec![23]);
//!
//! let n3 = Base95::avg_with_one(&n1);
//! assert_eq!(n3.to_string(), "g");
//! assert_eq!(n3.raw_digits(), vec![71]);
//!
//! let n4 = Base95::avg(&n1, &n2);
//! assert_eq!(n4.to_string(), "C");
//! assert_eq!(n4.raw_digits(), vec![35]);
//!
//! let n5 = Base95::from_str("j>Z= 4").unwrap();
//! assert_eq!(n5.raw_digits(), vec![74, 30, 58, 29, 0, 20]);
//! ```
//!
//! ## Why is `avg` imprecise?
//!
//! One of main considerations of this representation is storage efficiency of fractional index.
//! So it is better to have a little imprecise, shorter string, than perfectly precise, longer string.
//!
//! Of course, the result is deterministic, i.e., if the input is same, the output will always be same.

mod digits;

use crate::digits::Digits;

const ASCII_MIN: u8 = 32; // space

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Base95(String);

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    InvalidChar,
    EmptyNotAllowed,
}

impl Base95 {
    /// Create a fractional number of base 95, which represents `47 / 95`.
    /// The only way to create Base95 instance without any arguments.
    pub fn mid() -> Self {
        Digits::mid().into()
    }

    pub fn avg(lhs: &Self, rhs: &Self) -> Self {
        Digits::avg(&lhs.into(), &rhs.into()).into()
    }

    pub fn avg_with_zero(n: &Self) -> Self {
        Digits::avg(&Digits::zero(), &n.into()).into()
    }

    pub fn avg_with_one(n: &Self) -> Self {
        Digits::avg(&Digits::one(), &n.into()).into()
    }

    pub fn raw_digits(&self) -> Vec<u8> {
        Digits::from(self).0
    }
}

impl ToString for Base95 {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl std::str::FromStr for Base95 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseError::EmptyNotAllowed)
        } else if s.chars().any(|c| !c.is_ascii() || c.is_ascii_control()) {
            Err(ParseError::InvalidChar)
        } else {
            Ok(Base95(s.to_owned()))
        }
    }
}

impl From<Digits> for Base95 {
    fn from(digits: Digits) -> Self {
        Self(String::from_utf8(digits.0.iter().map(|x| x + ASCII_MIN).collect()).unwrap())
    }
}

impl From<&Base95> for Digits {
    fn from(base95: &Base95) -> Self {
        Self(base95.0.as_bytes().iter().map(|x| x - ASCII_MIN).collect())
    }
}

impl From<Base95> for String {
    fn from(base95: Base95) -> Self {
        base95.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        assert_eq!(Base95::from_str(""), Err(ParseError::EmptyNotAllowed));
        assert_eq!(Base95::from_str("한글"), Err(ParseError::InvalidChar));
        assert_eq!(Base95::from_str("O").unwrap(), Base95::mid());
    }
}
