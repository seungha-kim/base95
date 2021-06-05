use base95::Base95;
use rand::prelude::*;

fn main() {
    let mut v = vec![Base95::mid()];
    let mut rng = thread_rng();

    for _ in 0..1000 {
        let pos: usize = rng.gen_range(0..=v.len());
        if pos == 0 {
            v.insert(pos, Base95::avg_with_zero(&v[pos]));
        } else if pos == v.len() {
            v.insert(pos, Base95::avg_with_one(&v[pos - 1]));
        } else {
            v.insert(pos, Base95::avg(&v[pos - 1], &v[pos]));
        }
    }

    for n in &v {
        println!("{}", n.to_string());
    }
    println!("max len: {:?}", v.iter().map(|n| n.to_string().len()).max())
}
