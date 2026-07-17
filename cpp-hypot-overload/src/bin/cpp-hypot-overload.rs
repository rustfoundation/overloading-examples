//! Example of calling overloaded C++ arithmetic functions from Rust.
#![feature(splat, tuple_trait)]
#![expect(incomplete_features)]

use cpp::cpp;
use std::ffi::{c_double, c_int};
use std::marker::Tuple;

cpp! {{
    #include <cmath>
}}

/// The arguments of an overloaded C++ `hypot` function.
trait HypotArgs: Tuple {
    type Output;
    fn call_hypot(self) -> Self::Output;
}

/// Calls the overloaded C++ `std::hypot` function with the given arguments.
fn hypot<Args: HypotArgs>(#[splat] args: Args) -> <Args as HypotArgs>::Output {
    args.call_hypot()
}

impl HypotArgs for (c_double, c_double) {
    type Output = c_double;

    fn call_hypot(self) -> c_double {
        let (x, y) = self;
        unsafe {
            cpp!([x as "double", y as "double"] -> c_double as "double" {
                // This is C++ code!
                return std::hypot(x, y);
            })
        }
    }
}

impl HypotArgs for (c_double, c_double, c_double) {
    type Output = c_double;

    fn call_hypot(self) -> c_double {
        let (x, y, z) = self;
        unsafe {
            cpp!([x as "double", y as "double", z as "double"] -> c_double as "double" {
                return std::hypot(x, y, z);
            })
        }
    }
}

impl HypotArgs for (c_int, c_int) {
    type Output = c_double;

    fn call_hypot(self) -> c_double {
        let (x, y) = self;
        unsafe {
            cpp!([x as "int", y as "int"] -> c_double as "double" {
                // C++ instantiates the `std::hypot<int, int>` template, which casts to `double`
                // before calling `std::hypot(double, double)`.
                return std::hypot(x, y);
            })
        }
    }
}

impl HypotArgs for (c_int, c_int, c_int) {
    type Output = c_double;

    fn call_hypot(self) -> c_double {
        let (x, y, z) = self;
        unsafe {
            cpp!([x as "int", y as "int", z as "int"] -> c_double as "double" {
                return std::hypot(x, y, z);
            })
        }
    }
}

fn main() {
    // Rust doesn't have c_* literal type specifiers.
    println!("|(3, 4)|    double = {}", hypot(3.0_f64, 4.0_f64));
    println!("|(2, 3, 6)| double = {}", hypot(2.0_f64, 3.0_f64, 6.0_f64));
    println!("|(3, 4)|    int    = {}", hypot(3_i32, 4_i32));
    println!("|(2, 3, 6)| int    = {}", hypot(2_i32, 3_i32, 6_i32));
}
