use rand::prelude::*;
use rand_xoshiro::Xoshiro256Plus;
use squareness::{is_square, is_square_by_sqrt};

fn test_small_generic(is_square: impl Fn(u64) -> bool) {
    let mut next_sqrt = 0_u64;
    let mut next_square = 0_u64;
    for x in 0_u64..(1 << 20) {
        if x == next_square {
            assert!(is_square(x), "is_square({}) should be true", x);
            next_sqrt += 1;
            next_square = next_sqrt * next_sqrt;
        } else {
            assert!(!is_square(x), "is_square({}) should be false", x);
        }
    }
}

fn test_large_squares_generic(is_square: impl Fn(u64) -> bool) {
    let mut rng = Xoshiro256Plus::seed_from_u64(0);
    for _ in 0..10000 {
        let xrt = rng.next_u32() as u64;
        let x = xrt * xrt;
        assert!(is_square(x), "is_square({}) should be true", x);
    }
}

fn test_large_generic(is_square1: impl Fn(u64) -> bool, is_square2: impl Fn(u64) -> bool) {
    let mut rng = Xoshiro256Plus::seed_from_u64(0);
    for _ in 0..10000 {
        let x = rng.next_u64();
        let result1 = is_square1(x);
        let result2 = is_square2(x);
        assert_eq!(
            result1, result2,
            "is_square1({}) is {:?} but is_square2({}) is {:?}",
            x, result1, x, result2,
        );
    }
}

#[test]
fn test_small() {
    test_small_generic(is_square);
}

#[test]
fn test_large_squares() {
    test_large_squares_generic(is_square);
}

#[test]
fn test_small_sqrt() {
    test_small_generic(is_square_by_sqrt);
}

#[test]
fn test_large_squares_sqrt() {
    test_large_squares_generic(is_square_by_sqrt);
}

#[test]
fn test_large() {
    test_large_generic(is_square, is_square_by_sqrt);
}
