/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(not(feature = "std"), no_std)]

mod bindings;

#[cfg(not(feature = "std"))]
mod disambiguate {
    extern crate alloc;
    pub use alloc::string::FromUtf16Error;
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
}

#[cfg(feature = "std")]
mod disambiguate {
    pub use std::string::FromUtf16Error;
}

use disambiguate::*;

mod decode;
use decode::*;

mod bstr;
pub use bstr::*;

mod hstring;
pub use hstring::*;
