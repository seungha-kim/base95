pub mod digits;

use crate::digits::Digits;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Base95(String);

const ASCII_MIN: u8 = 32; // space

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    InvalidChar,
    EmptyNotAllowed,
}

impl Base95 {
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
