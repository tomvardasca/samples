extern crate num;
extern crate rayon;
extern crate time;
use rayon::prelude::*;
use std::env;

pub fn fibonacci_rec(n: i32) -> u64 {
    if n < 0 {
        return 0;
    }
    match n {
        0 => 0,
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}

pub fn fibonacci_iter(n: i32) -> u64 {
    if n < 0 {
        return 0;
    } else if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

const REPEAT: u32 = 4500;

fn main() {
    let fib_seq_elements = vec![2, 3, 4, 6, 8, 10, 11, 13, 15, 18, 20, 23, 25];
    let mut fib_seq: Vec<i32> = Vec::new();
    for _i in 0..REPEAT {
        fib_seq.extend(&fib_seq_elements);
    }
    let args: Vec<String> = env::args().collect();
    let function = &args[1];
    if function == "fib_iter" {
        let _a = (&fib_seq).into_iter().map(|i| fibonacci_iter(*i));
        println!("{}", _a.len());
    } else if function == "fib_rec" {
        let _a = (&fib_seq).into_iter().map(|i| fibonacci_rec(*i));
        println!("{}", _a.len());
    } else if function == "fib_iter_par" {
        let _a = (&fib_seq).par_iter().map(|i| fibonacci_iter(*i));
        println!("{}", _a.len());
    } else if function == "fib_rec_par" {
        let _a = (&fib_seq).par_iter().map(|i| fibonacci_rec(*i));
        println!("{}", _a.len());
    }
}
