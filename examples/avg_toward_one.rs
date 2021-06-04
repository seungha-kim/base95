use base95::digits::Digits;
use base95::Base95;

fn main() {
    let mut working = Digits::new();
    let one = Digits::one();
    for _ in 0..30 {
        working = Digits::avg(&working, &one);
        println!("{:?}", working);
        println!("{:?}", Base95::from(working.clone()));
    }
}
