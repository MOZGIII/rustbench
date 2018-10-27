#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

const ARENA_SIZE: u64 = 10_000_000;

#[bench]
fn bench_random_access_no_vx(b: &mut Bencher) {
    use std::iter::FromIterator;
    let mut x = Vec::from_iter(0..ARENA_SIZE);
    b.iter(|| {
        for i in 0..x.len() {
            x[i] += 123;
        }
    });
}

#[bench]
fn bench_simple_for_loop(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ARENA_SIZE {
            black_box(());
        }
    });
}

#[bench]
fn bench_simple_for_loop_with_loop_var(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..ARENA_SIZE {
            black_box(i);
        }
    });
}

#[bench]
fn bench_simple_for_loop_with_global_var(b: &mut Bencher) {
    b.iter(|| {
        let mut n: u64 = 0;
        for _ in 0..ARENA_SIZE {
            if n % 2 == 0 {
                n += 1;
            } else {
                n -= 1;
            }
        }
        n
    });
}
