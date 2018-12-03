#[macro_use]
extern crate neon;
extern crate num;
extern crate rayon;
extern crate time;
use num::{BigUint, One};
use rayon::prelude::*;
use std::ops::Mul;
use neon::vm::{Call, JsResult};
use neon::js::{JsString,JsNumber};

fn factorial(call: Call) -> JsResult<JsString> {
    let n = call.arguments.require(call.scope, 0)?.check::<JsNumber>()?.value() as u32;
    Ok(JsString::new(call.scope, (1..n + 1).map(BigUint::from).fold(BigUint::one(), Mul::mul).to_str_radix(10).as_str()).unwrap())
}

fn factorial_while(call: Call) -> JsResult<JsString> {
  let mut num = call.arguments.require(call.scope, 0)?.check::<JsNumber>()?.value() as u32;
  let mut result = num;
  if num == 0 || num == 1 {
    return Ok(JsString::new(call.scope, "1").unwrap());
  }
  while num > 1 { 
    num -= 1;
    result *= num;
  }
  return Ok(JsString::new(call.scope, &result.to_string()).unwrap());
}

fn factorial_recursion(call: Call) -> JsResult<JsString> {
    let num = call.arguments.require(call.scope, 0)?.check::<JsNumber>()?.value() as u32;
    fn product(a: u32, b: u32) -> BigUint {
        if a == b {
            return a.into();
        }
        let mid = (a + b) / 2;
        product(a, mid) * product(mid + 1, b)
    }

    Ok(JsString::new(call.scope, product(1, num).to_str_radix(10).as_str()).unwrap())
}

fn factorial_par_iter(call: Call) -> JsResult<JsString> {
    let num = call.arguments.require(call.scope, 0)?.check::<JsNumber>()?.value() as u32;
    fn fact(n: u32) -> BigUint {
        (1..n + 1)
            .into_par_iter()
            .map(BigUint::from)
            .reduce_with(Mul::mul)
            .unwrap()
    }

    Ok(JsString::new(call.scope, fact(num).to_str_radix(10).as_str()).unwrap())
}

register_module!(m, {
    m.export("factorial", factorial)?;
    m.export("factorial_while", factorial_while)?;
    m.export("factorial_recursion", factorial_recursion)?;
    m.export("factorial_par_iter", factorial_par_iter)?;
    Ok(())
});
