const BASE: u8 = 95;
const MID: u8 = BASE / 2;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Digits(pub(crate) Vec<u8>);

/// represents rational number of base 95 between 0 and 1. leading "0." is omitted.
impl Digits {
    pub fn mid() -> Self {
        Self(vec![MID])
    }

    pub fn avg(lhs: &Self, rhs: &Self) -> Self {
        // NOTE: u8 has enough capacity which can contain double of 95
        let half = Self::naive_add(&lhs, &rhs).half();

        let (larger, smaller) = if lhs > rhs { (lhs, rhs) } else { (rhs, lhs) };
        for i in 1..half.0.len() {
            // TODO: optimize not to heap allocate
            let trimmed = Self(half.0[..i].to_vec()).normalized();
            if smaller < &trimmed && &trimmed < larger {
                return trimmed;
            }
        }
        half
    }

    pub fn zero() -> Self {
        Self(vec![])
    }

    pub fn one() -> Self {
        Self(vec![95])
    }

    fn half(&self) -> Self {
        if self.0.is_empty() {
            return self.clone();
        }

        let mut result = Vec::new();
        let mut memory: u8 = 0;

        for i in &self.0 {
            memory = memory * BASE + i;
            let div = memory / 2;
            let rem = memory % 2;
            result.push(div);

            memory = rem;
        }

        if memory != 0 {
            result.push(BASE / 2);
        }

        Self(result)
    }

    /// naive_add simply adds each digits. it can result in invalid instance so needs post-process (e.g. half)
    fn naive_add(lhs: &Self, rhs: &Self) -> Self {
        let (Digits(longer), Digits(shorter)) = if lhs.0.len() > rhs.0.len() {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };

        Self(
            longer
                .iter()
                .zip(shorter.iter().chain(std::iter::repeat(&0)))
                .map(|(l, r)| l + r)
                .collect::<Vec<_>>(),
        )
    }

    fn normalized(mut self) -> Self {
        // remove trailing zeros
        loop {
            match self.0.last() {
                Some(d) if d == &0 => {
                    self.0.pop();
                }
                _ => {
                    break;
                }
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half() {
        assert_eq!(Digits(vec![2]).half(), Digits(vec![1]));
        assert_eq!(Digits(vec![1]).half(), Digits(vec![0, 47]));
        assert_eq!(Digits(vec![2, 2]).half(), Digits(vec![1, 1]));
        assert_eq!(Digits(vec![2, 1]).half(), Digits(vec![1, 0, 47]));
    }

    #[test]
    fn naive_add() {
        assert_eq!(
            Digits::naive_add(&Digits(vec![1]), &Digits(vec![1])),
            Digits(vec![2])
        );

        assert_eq!(
            Digits::naive_add(&Digits(vec![0, 47]), &Digits(vec![0, 48])),
            Digits(vec![0, 95])
        );

        assert_eq!(
            Digits::naive_add(&Digits(vec![1]), &Digits(vec![0, 0, 1])),
            Digits(vec![1, 0, 1])
        );
    }

    #[test]
    fn avg() {
        assert_eq!(
            Digits::avg(&Digits(vec![1]), &Digits(vec![1])),
            Digits(vec![1])
        );

        assert_eq!(
            Digits::avg(&Digits(vec![2]), &Digits(vec![2])),
            Digits(vec![2])
        );

        assert_eq!(
            Digits::avg(&Digits(vec![2]), &Digits(vec![2])),
            Digits(vec![2])
        );

        assert_eq!(
            Digits::avg(&Digits(vec![47]), &Digits(vec![48])),
            Digits(vec![47, 47])
        );

        assert_eq!(
            Digits::avg(&Digits(vec![0]), &Digits(vec![1])),
            Digits(vec![0, 47])
        );
    }
}
