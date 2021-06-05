use base95::Base95;

fn main() {
    let mut working = Base95::mid();
    for _ in 0..30 {
        working = Base95::avg_with_one(&working);
        println!("{:?}", working);
    }
}
