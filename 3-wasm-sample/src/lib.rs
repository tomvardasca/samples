
#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate num;
extern crate rayon;
extern crate time;
use num::{BigUint, One};
use rayon::prelude::*;
use std::ops::Mul;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn factorial(n: u32) -> String {
    (1..n + 1).map(BigUint::from).fold(BigUint::one(), Mul::mul).to_str_radix(10)
}

#[wasm_bindgen]
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

#[wasm_bindgen]
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

#[wasm_bindgen]
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