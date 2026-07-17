//! Example of Rust arithmetic via overloaded functions.

#![feature(splat, tuple_trait)]
#![expect(incomplete_features)]

use std::marker::Tuple;

/// The arguments of an overloaded Rust `hypot` function.
trait HypotArgs: Tuple {
    type Output;
    fn call_hypot(self) -> Self::Output;
}

/// Calls the overloaded Rust `hypot` function with the given arguments.
fn hypot<Args: HypotArgs>(#[splat] args: Args) -> <Args as HypotArgs>::Output {
    args.call_hypot()
}

impl HypotArgs for (f64, f64) {
    type Output = f64;

    fn call_hypot(self) -> f64 {
        let (x, y) = self;
        x.hypot(y)
    }
}

impl HypotArgs for (f64, f64, f64) {
    type Output = f64;

    fn call_hypot(self) -> f64 {
        let (x, y, z) = self;
        // Rust std doesn't have a `hypot(x, y, z)` implementation.
        f64::sqrt(x.powi(2) + y.powi(2) + z.powi(2))
    }
}

impl HypotArgs for (i32, i32) {
    type Output = f64;

    fn call_hypot(self) -> f64 {
        let (x, y) = self;
        f64::from(x).hypot(f64::from(y))
    }
}

impl HypotArgs for (i32, i32, i32) {
    type Output = f64;

    fn call_hypot(self) -> f64 {
        let (x, y, z) = self;
        f64::sqrt(f64::from(x).powi(2) + f64::from(y).powi(2) + f64::from(z).powi(2))
    }
}

fn main() {
    println!("|(3, 4)|    f64 = {}", hypot(3.0_f64, 4.0_f64));
    println!("|(2, 3, 6)| f64 = {}", hypot(2.0_f64, 3.0_f64, 6.0_f64));
    println!("|(3, 4)|    i32 = {}", hypot(3_i32, 4_i32));
    println!("|(2, 3, 6)| i32 = {}", hypot(2_i32, 3_i32, 6_i32));
}
