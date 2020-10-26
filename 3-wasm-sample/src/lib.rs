#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]

extern crate num;
extern crate rayon;
extern crate time;
extern crate wasm_bindgen;
use num::{BigUint, One};
use rayon::prelude::*;
use std::ops::Mul;
use wasm_bindgen::prelude::*;

const REPEAT: u32 = 4500;

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

#[wasm_bindgen]
pub fn fib_iter() -> i32 {
  let fib_seq_elements = vec![2, 3, 4, 6, 8, 10, 11, 13, 15, 18, 20, 23, 25];
  let mut fib_seq: Vec<i32> = Vec::new();
  for _i in 0..REPEAT {
    fib_seq.extend(&fib_seq_elements);
  }
  let _a = (&fib_seq).into_iter().map(|i| fibonacci_iter(*i));
  _a.len() as i32
}

#[wasm_bindgen]
pub fn fib_rec() -> i32 {
  let fib_seq_elements = vec![2, 3, 4, 6, 8, 10, 11, 13, 15, 18, 20, 23, 25];
  let mut fib_seq: Vec<i32> = Vec::new();
  for _i in 0..REPEAT {
    fib_seq.extend(&fib_seq_elements);
  }
  let _a = (&fib_seq).into_iter().map(|i| fibonacci_rec(*i));
  _a.len() as i32
}

#[wasm_bindgen]
pub fn fib_iter_par() -> i32 {
  let fib_seq_elements = vec![2, 3, 4, 6, 8, 10, 11, 13, 15, 18, 20, 23, 25];
  let mut fib_seq: Vec<i32> = Vec::new();
  for _i in 0..REPEAT {
    fib_seq.extend(&fib_seq_elements);
  }
  let _a = (&fib_seq).par_iter().map(|i| fibonacci_iter(*i));
  _a.len() as i32
}

#[wasm_bindgen]
pub fn fib_rec_par() -> i32 {
  let fib_seq_elements = vec![2, 3, 4, 6, 8, 10, 11, 13, 15, 18, 20, 23, 25];
  let mut fib_seq: Vec<i32> = Vec::new();
  for _i in 0..REPEAT {
    fib_seq.extend(&fib_seq_elements);
  }
  let _a = (&fib_seq).par_iter().map(|i| fibonacci_rec(*i));
  _a.len() as i32
}
