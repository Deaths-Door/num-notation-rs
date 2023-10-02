#![doc = include_str!("../README.md")]

#![forbid(
    missing_docs,
    unsafe_code,
    unused_imports,
    unused_mut,
    unused_results,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,
    trivial_casts,
    unsafe_op_in_unsafe_fn,
    overflowing_literals,
)]

pub use standardform;
pub use fraction;

mod number;
mod err;

pub use number::*;
pub use err::*;

#[cfg(feature = "num")]
mod num;

#[cfg(feature = "num")]
pub use self::num::*;

#[cfg(feature="num")]
pub use num_traits::*;

#[cfg(feature = "hash")]
mod hash;

#[cfg(feature = "hash")]
pub use self::hash::*;

#[cfg(feature = "nom")]
mod nom;

#[cfg(feature = "nom")]
pub use nom::*;