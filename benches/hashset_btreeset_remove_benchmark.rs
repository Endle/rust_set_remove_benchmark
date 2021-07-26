use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::rngs::StdRng;
use rand::{Rng,SeedableRng};
use rand::prelude::SliceRandom;
use std::collections::{HashSet, BTreeSet};

fn hashset_test(n: usize) -> () {
    let (push, pop) = generate_random_operations(n);
    let mut set = HashSet::with_capacity(n);
    for x in push {
        set.insert(x);
    }
    for y in pop {
        set.remove(&y);
    }
}

fn btreeset_test(n: usize) -> () {
    let (push, pop) = generate_random_operations(n);
    let mut set = BTreeSet::new();
    for x in push {
        set.insert(x);
    }
    for y in pop {
        set.remove(&y);
    }
}



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

fn bench_set_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Set_Remove");

    for i in [10, 100, 500, 1000, 5000, 10000,
        25_000, 50_000, 100_000, 200_000, 300_000, 400_000, 500_000].iter() {
        group.bench_with_input(BenchmarkId::new("Hashset", i), i,
                               |b, i| b.iter(|| hashset_test(*i)));
        group.bench_with_input(BenchmarkId::new("BTree", i), i,
                               |b, i| b.iter(|| btreeset_test(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench_set_pop);
criterion_main!(benches);