//! Example of calling overloaded C++ arithmetic functions from Rust.

#![feature(splat, tuple_trait)]
#![expect(incomplete_features)]

use cpp::cpp;
use std::ffi::{c_double, c_float, c_int};
use std::marker::Tuple;

cpp! {{
    #include <cmath>
    #include <iostream>
}}

/// The arguments of an overloaded C++ `pow` function.
trait PowArgs: Tuple {
    type Output;
    fn call_pow(self) -> Self::Output;
}

/// Calls the overloaded C++ `std::pow` function with the given arguments.
fn pow<Args: PowArgs>(#[splat] args: Args) -> <Args as PowArgs>::Output {
    args.call_pow()
}

impl PowArgs for (c_float, c_float) {
    type Output = c_float;

    fn call_pow(self) -> c_float {
        let (base, exponent) = self;
        unsafe {
            cpp!([base as "float", exponent as "float"] -> c_float as "float" {
                // This is C++ code!
                return std::pow(base, exponent);
            })
        }
    }
}

impl PowArgs for (c_double, c_double) {
    type Output = c_double;

    fn call_pow(self) -> c_double {
        let (base, exponent) = self;
        unsafe {
            cpp!([base as "double", exponent as "double"] -> c_double as "double" {
                return std::pow(base, exponent);
            })
        }
    }
}

impl PowArgs for (c_int, c_int) {
    type Output = c_double;

    fn call_pow(self) -> c_double {
        let (base, exponent) = self;
        unsafe {
            cpp!([base as "int", exponent as "int"] -> c_double as "double" {
                // C++ casts to `double` before calling `std::pow<double>`
                return std::pow(base, exponent);
            })
        }
    }
}

fn main() {
    println!("2^3 f32 = {}", pow(2.0_f32, 3.0_f32));
    println!("2^3 f64 = {}", pow(2.0_f64, 3.0_f64));
    println!("2^3 i32 = {}", pow(2_i32, 3_i32));
}
