#![allow(warnings)]

#[cfg(feature = "8")]
mod bindings_8;

#[cfg(feature = "9")]
mod bindings_9;

pub mod binding {
    #[cfg(feature = "8")]
    pub use crate::bindings_8::*;

    #[cfg(feature = "9")]
    pub use crate::bindings_9::*;
}