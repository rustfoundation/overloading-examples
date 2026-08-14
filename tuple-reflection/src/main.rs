//! Demonstrates combining the type_info and splat features for creating functions that can accept
//! arbitrarily large numbers of arguments.
//! We also use various const features to move runtime checks into compiletime.
//!
//! In this example, we're recreating [`min`](core::cmp::min) but able to accept
//! unlimited arguments. A natural instinct would be to reach for `From<(T, ..)> for [T; _]`,
//! however this is only implemented for up to twelve elements, and not at all
//! in const yet.

#![allow(incomplete_features)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_trait_impl)]
#![feature(splat)]
#![feature(tuple_trait)]
#![feature(type_info)]

use core::marker::{Destruct, PhantomData, Tuple};
use core::mem::MaybeUninit;
use core::mem::type_info::{Type, TypeKind};

/// This is a simple version of what we're trying to create, but it has several issues:
///
/// 1. Not const, because `From<(T, ..)>  for [T; N]` is not const.
/// 2. Requires a redundant `N` parameter since we cannot use the tuple arity to infer
///    the array's length.
/// 3. `From<(T, ..)>  for [T; N]` is only implemented up to 12 elements
pub fn min_simple<const N: usize, T: Ord>(
    x: T,
    #[rustc_splat] args: impl Tuple + Into<[T; N]>,
) -> T {
    args.into().into_iter().fold(x, Ord::min)
}

/// This is what we'll make instead.
pub const fn min<T: [const] Destruct + [const] Ord>(x: T, #[rustc_splat] args: impl Tuple) -> T {
    TupleArray::from(args).fold(x, Ord::min)
}

/// This type is a bridge between homogeneous tuples and arrays.
/// While the `A` parameter implies it can accept any [`Tuple`],
/// the only way to construct this type is through [`new`](TupleArray::new),
/// which const-asserts the tuple is homogeneous and only includes the type `T`.
pub struct TupleArray<T, A: Tuple> {
    /// Because we're moving one item at a time out of the tuple, it will spend
    /// most of its life in an invalid state, so we must wrap it in [`MaybeUninit`].
    tuple: MaybeUninit<A>,
    index: usize,
    _phantom: PhantomData<T>,
}

/// We may drop [`TupleArray`] before moving all items out of it.
/// During drop, we'll remove the rest, if any.
const impl<T: [const] Destruct, A: Tuple> Drop for TupleArray<T, A> {
    fn drop(&mut self) {
        while self.next().is_some() {}
    }
}

const impl<T, A: Tuple> Iterator for TupleArray<T, A> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // While a homogeneous tuple looks like an array, we aren't guaranteed they
        // have the same layout, so we use type_info to retrieve the offsets of
        // each item within the tuple.
        let info = const {
            let TypeKind::Tuple(info) = Type::of::<A>().kind else {
                unreachable!()
            };
            info
        };

        if self.index >= info.fields.len() {
            return None;
        }

        // SAFETY:
        // * A is a tuple of the form (T, ... T), where each field is aligned
        // * info provides accurate offsets for each field
        // * self.index points to a field we have yet to read from
        let x = unsafe {
            self.tuple
                .as_ptr()
                .byte_add(info.fields[self.index].offset)
                .cast::<T>()
                .read()
        };
        self.index += 1;

        Some(x)
    }
}

/// While it looks like we can turn any tuple into a [`TupleArray`], this trait
/// can only be successfully implemented for tuples where all fields are equivalent
/// and of the type `T`. Since we can't (yet) express this in the type system,
/// we use panic within a const to fail at compiletime if attempting to call this
/// on an invalid tuple.
const impl<T, A: Tuple> From<A> for TupleArray<T, A> {
    fn from(tuple: A) -> Self {
        // We use a runtime assert over a const value to ensure the constant is evaluated.
        // This wont ever panic at runtime, only compiletime.
        assert!(
            const {
                let TypeKind::Tuple(a_info) = Type::of::<A>().kind else {
                    unreachable!()
                };
                let TypeKind::Tuple(t_info) = Type::of::<(T,)>().kind else {
                    unreachable!()
                };
                let mut i = 1;
                while i < a_info.fields.len() {
                    if a_info.fields[i].ty != a_info.fields[i - 1].ty {
                        panic!("Not a homogeneous tuple");
                    }
                    i += 1;
                }
                if !a_info.fields.is_empty() && t_info.fields[0].ty != a_info.fields[0].ty {
                    panic!("Not a homogeneous tuple of the required type");
                }
                true
            }
        );
        Self {
            tuple: MaybeUninit::new(tuple),
            index: 0,
            _phantom: PhantomData,
        }
    }
}

fn main() {
    dbg!(min_simple(1, 2, 3, 4));
    // the 14th item will cause this to fail
    dbg!(min_simple(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, /* 14 */
    ));

    dbg!(min(1, 2, 3, 4));
    // our type_info based alternative is able to accept an arbitrary number of
    // arguments, but practically compilation times will become prohibitively long
    // around 15,000 arguments.
    dbg!(min(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    ));
}
