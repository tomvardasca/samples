extern crate num;
extern crate rayon;
extern crate time;
use num::{BigUint, One};
use rayon::prelude::*;
use std::env;
use std::ops::Mul;
use std::time::Instant;

pub fn factorial(n: u32) -> String {
    (1..n + 1)
        .map(BigUint::from)
        .fold(BigUint::one(), Mul::mul)
        .to_str_radix(10)
}

pub fn factorial_recursion(num: u32) -> String {
    fn product(a: u32, b: u32) -> BigUint {
        if a == b {
            return a.into();
        }
        let mid = (a + b) / 2;
        product(a, mid) * product(mid + 1, b)
    }

    return product(1, num).to_str_radix(10);
}

pub fn factorial_par_iter(num: u32) -> String {
    fn fact(n: u32) -> BigUint {
        (1..n + 1)
            .into_par_iter()
            .map(BigUint::from)
            .reduce_with(Mul::mul)
            .unwrap()
    }

    return fact(num).to_str_radix(10);
}

pub fn factorial_while(mut num: u32) -> String {
    let mut result = num;
    if num == 0 || num == 1 {
        return "1".to_string();
    }
    while num > 1 {
        num -= 1;
        result *= num;
    }
    return result.to_string();
}

fn main() {
    println!("pure rust:");
    let mut bench_size: u32 = 50000;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        bench_size = args[1].parse::<u32>().unwrap();
    }
    let start = Instant::now();
    println!("factorial_while:");
    factorial_while(bench_size);
    println!(
        "{} miliseconds.",
        start.elapsed().subsec_micros() as f32 / 1000.0
    );

    let start = Instant::now();
    println!("factorial_iterator:");
    factorial(bench_size);
    println!(
        "{} miliseconds.",
        start.elapsed().subsec_micros() as f32 / 1000.0
    );

    let start = Instant::now();
    println!("factorial_recursion:");
    factorial_recursion(bench_size);
    println!(
        "{} miliseconds.",
        start.elapsed().subsec_micros() as f32 / 1000.0
    );

    let start = Instant::now();
    println!("factorial_par_iter:");
    factorial_par_iter(bench_size);
    println!(
        "{} miliseconds.",
        start.elapsed().subsec_micros() as f32 / 1000.0
    );
}
