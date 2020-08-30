#![feature(test)]

extern crate test;

use test::{Bencher, black_box};
use squareness::{is_square, is_square_by_sqrt};

#[bench]
fn bench_is_square_20(b: &mut Bencher) {
    b.iter(|| is_square(black_box(20)));
}

#[bench]
fn bench_is_square_25(b: &mut Bencher) {
    b.iter(|| is_square(black_box(25)));
}

#[bench]
fn bench_is_square_by_sqrt_20(b: &mut Bencher) {
    b.iter(|| is_square_by_sqrt(black_box(20)));
}

#[bench]
fn bench_is_square_by_sqrt_25(b: &mut Bencher) {
    b.iter(|| is_square_by_sqrt(black_box(25)));
}
