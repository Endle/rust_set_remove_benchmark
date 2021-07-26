fn main() {
    println!("Hello, world!");
    for _ in 0..5 {
        let (push, pop) = generate_random_operations(10);
        println!{"{:?}",&push};
        println!{"{:?}", &pop};
        println!("===========")
    }
}

use rand::rngs::StdRng;
use rand::{Rng,SeedableRng};
use rand::prelude::SliceRandom;

const RANDOM_RANGE:u32 = 1_000_000_000;
fn generate_random_operations(n: usize) ->(Vec<u32>, Vec<u32>) {
    let mut rng = StdRng::seed_from_u64(n as u64);
    let mut input = Vec::with_capacity(n);
    for _ in 0..n {
        input.push(rng.gen_range(0..RANDOM_RANGE));
    }
    let mut pop = input.clone();
    pop.shuffle(&mut rng);
    (input, pop)
}