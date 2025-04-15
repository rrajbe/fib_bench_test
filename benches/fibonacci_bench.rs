#![feature(test)]

extern crate test;

use fib_bench_test::fibonacci::{fibonacci_loop, fibonacci_rec};
use test::Bencher;

#[bench]
fn bench_fibonacci_rec_ten(b: &mut Bencher) {
    b.iter(|| fibonacci_rec(10));
}

#[bench]
fn bench_fibonacci_loop_ten(b: &mut Bencher) {
    b.iter(|| fibonacci_loop(10));
}
