use base95::digits::Digits;
use base95::Base95;

fn main() {
    let mut working = Digits::new();
    let zero = Digits::zero();
    for _ in 0..30 {
        working = Digits::avg(&working, &zero);
        println!("{:?}", working);
        println!("{:?}", Base95::from(working.clone()));
    }
}
