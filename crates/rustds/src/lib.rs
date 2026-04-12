#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod tds;

#[cfg(feature = "smp")]
pub mod smp;

#[cfg(feature = "api")]
pub mod interface;