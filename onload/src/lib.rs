#![allow(warnings)]

#[cfg(feature = "release")]
pub mod binding;

#[cfg(feature = "debug")]
pub mod ge;
