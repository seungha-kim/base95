use base95::digits::Digits;
use base95::Base95;

fn main() {
    for i in 0..95 {
        println!(
            "{} - {}",
            i,
            Base95::from(Digits::with_raw_digits(vec![i])).to_string()
        );
    }
}
