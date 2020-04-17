#![feature(const_int_pow)]
extern crate rand;
use std::time::{Instant};
const MIN_VAL: i64 = 1;
const MAX_VAL: i64 = 10_i64.pow(6);

use std::io::stdin;

fn main() {

    let cases: i64 = read_int();

    for _ in 0..cases {
        let limit: i64 = read_int();
        println!("{}", do_it(limit));
    }

    let start = Instant::now();
    let result = do_it(MAX_VAL);
    let end = Instant::now();
    println!("time: {:?}", end - start);
    println!("res: {:?}", result);
}

fn do_it(limit: i64) -> i64 {
    limit
}

//max 10^18
fn read_int() -> i64 {
    let mut nr_str = String::new();
    
    stdin()
        .read_line(&mut nr_str)
        .expect("read failed");
    
    let nr: i64 = nr_str.trim().parse().expect("invalid input");

   nr
}


// adapted from num crate https://github.com/rust-num/num
pub fn fast_pow(mut base: i64, mut exp: i64) -> i64 {
    if exp == 0 { return 1 }

    while exp & 1 == 0 {
        base = base.clone() * base;
        exp >>= 1;
    }
    if exp == 1 { return base }

    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = base.clone() * base;
        if exp & 1 == 1 {
            acc = acc * base.clone();
        }
    }

    acc
}


#[test]
fn do_it_works() {
    assert_eq!(do_it(2), 2);
}

#[test]
fn test_min() {
    assert_eq!(do_it(MIN_VAL), MIN_VAL);
}

#[test]
fn test_max() {
    assert_eq!(do_it(MAX_VAL), MAX_VAL);
}

#[test]
fn fuzz_do_it() {
    use rand::distributions::{Uniform, Distribution};

    let iters = 100;

    let dist = Uniform::from(MIN_VAL..MAX_VAL);
    let mut rng = rand::thread_rng();

    for _ in 0..iters {
        let rand_int = dist.sample(&mut rng);
        assert_eq!(do_it(rand_int), rand_int);
    }
}
