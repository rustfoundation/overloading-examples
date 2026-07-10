//! Example of Rust arithmetic via overloaded functions.

#![feature(splat, tuple_trait)]
#![expect(incomplete_features)]

use std::marker::Tuple;

/// The arguments of an overloaded Rust power function.
trait PowArgs: Tuple {
    type Output;
    fn call_pow(self) -> Self::Output;
}

/// Calls the specific Rust power function with the given arguments.
fn pow<Args: PowArgs>(#[splat] args: Args) -> <Args as PowArgs>::Output {
    args.call_pow()
}

impl PowArgs for (f32, f32) {
    type Output = f32;

    fn call_pow(self) -> f32 {
        let (base, exponent) = self;
        base.powf(exponent)
    }
}

impl PowArgs for (f64, f64) {
    type Output = f64;

    fn call_pow(self) -> f64 {
        let (base, exponent) = self;
        base.powf(exponent)
    }
}

impl PowArgs for (i32, u32) {
    type Output = i32;

    fn call_pow(self) -> i32 {
        let (base, exponent) = self;
        // Rust does the calculations in integer arithmetic
        base.pow(exponent)
    }
}

fn main() {
    println!("2^3 f32 = {}", pow(2.0_f32, 3.0_f32));
    println!("2^3 f64 = {}", pow(2.0_f64, 3.0_f64));
    println!("2^3 i32 = {}", pow(2_i32, 3_u32));
}
