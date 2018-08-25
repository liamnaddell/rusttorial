extern crate num;
extern crate rayon;

use num::bigint::BigUint;
use num::traits::{Zero,One};
use rayon::prelude::*;
use std::ops::{Mul,Add};
use num::{CheckedDiv, CheckedSub};

/// Factorialn function, takes two numbers factorialn(5,2)=factorial(5)/factorial(2)
pub fn factorialn(num: u64, stop: u64) -> BigUint {
    (stop+1 .. num + 1).into_par_iter()
        .map(BigUint::from)
        .reduce_with(Mul::mul)
        .unwrap()
}

/// Factorialn but the stop is zero
pub fn factorial(num: u64) -> BigUint {
    if num == 0 {
        One::one()
    } else {
        factorialn(num, Zero::zero())
    }
}

/// The choose funciton(just look it up if you don't know what it means, I can't explain it here :(    )
pub fn c(top:u64,bottom:u64) -> Option<BigUint> {
    if bottom != Zero::zero() {
        let tminusb = CheckedSub::checked_sub(&top,&bottom)?;
        CheckedDiv::checked_div(&factorialn(top,tminusb),&factorial(bottom))
    } else {
        Some(One::one())
    }
}

/// Factorialn but with plus instead of multiply
pub fn plustorialn(num: u64, stop: u64) -> BigUint {
    plustorial(num)-plustorial(stop)
}

/// Factorial but with plus instead of multiply
pub fn plustorial(num: u64) -> BigUint {
    (0 .. num+1).into_par_iter()
        .map(BigUint::from)
        .reduce_with(Add::add)
        .unwrap()
}
