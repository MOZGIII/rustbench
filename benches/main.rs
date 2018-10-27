#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

const ARENA_SIZE: u64 = 10_000_000;

fn make_vx() -> Vec<u64> {
    use std::iter::FromIterator;
    Vec::from_iter((0..ARENA_SIZE).map(|_| 10))
}

#[bench]
fn bench_random_access(b: &mut Bencher) {
    use std::iter::FromIterator;
    let mut x = Vec::from_iter(0..ARENA_SIZE);
    let vx = make_vx();
    b.iter(|| {
        for i in 0..x.len() {
            x[i] += vx[i];
        }
    });
}

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
fn bench_ptr_element_access(b: &mut Bencher) {
    use std::iter::FromIterator;
    let mut x = Vec::from_iter(0..ARENA_SIZE);
    let vx = make_vx();
    b.iter(|| {
        let mut i = 0;
        for cx in &mut x {
            *cx += vx[i];
            i += 1;
        }
    });
}

#[bench]
fn bench_ptr_element_access_no_vx(b: &mut Bencher) {
    use std::iter::FromIterator;
    let mut x = Vec::from_iter(0..ARENA_SIZE);
    let vx = make_vx();
    b.iter(|| {
        let mut i = 0;
        for cx in &mut x {
            *cx += vx[i];
            i += 1;
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

#[bench]
fn bench_simple_vx_sum(b: &mut Bencher) {
    let vx = make_vx();
    b.iter(|| {
        let mut n: u64 = 0;
        for i in 0..ARENA_SIZE {
            n += vx[i as usize];
        }
        n
    });
}

#[bench]
fn bench_simple_vx_sum_and_internal_use(b: &mut Bencher) {
    let vx = make_vx();
    b.iter(|| {
        let mut n: u64 = 0;
        for i in 0..ARENA_SIZE {
            n += vx[i as usize];
            black_box(n);
        }
        n
    });
}
