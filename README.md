# base95

Textual representation of base 95 fractional numbers with arbitrary precision,
intended to be used in real-time collaborative applications.

[Documentation](https://docs.rs/base95)

## Example

```rust
use base95::Base95;
use std::str::FromStr;

fn main() {
    let n1 = Base95::mid();
    assert_eq!(n1.to_string(), "O");
    assert_eq!(n1.raw_digits(), vec![47]);

    let n2 = Base95::avg_with_zero(&n1);
    assert_eq!(n2.to_string(), "7");
    assert_eq!(n2.raw_digits(), vec![23]);

    let n3 = Base95::avg_with_one(&n1);
    assert_eq!(n3.to_string(), "g");
    assert_eq!(n3.raw_digits(), vec![71]);

    let n4 = Base95::avg(&n1, &n2);
    assert_eq!(n4.to_string(), "C");
    assert_eq!(n4.raw_digits(), vec![35]);

    let n5 = Base95::from_str("j>Z= 4").unwrap();
    assert_eq!(n5.raw_digits(), vec![74, 30, 58, 29, 0, 20]);
}
```