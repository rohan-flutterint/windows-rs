/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(not(feature = "std"), no_std)]

mod bindings;

mod decode;
use decode::*;

mod bstr;
pub use bstr::*;

mod hstring;
pub use hstring::*;

mod pcstr;
pub use pcstr::*;

mod pstr;
pub use pstr::*;

mod pcwstr;
pub use pcwstr::*;

mod pwstr;
pub use pwstr::*;

#[cfg(not(feature = "std"))]
mod disambiguate {
    extern crate alloc;
    pub use alloc::string::FromUtf16Error;
    pub use alloc::string::FromUtf8Error;
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
}

#[cfg(feature = "std")]
mod disambiguate {
    pub use std::string::FromUtf16Error;
    pub use std::string::FromUtf8Error;
}

use disambiguate::*;

extern "C" {
    fn strlen(s: PCSTR) -> usize;
    fn wcslen(s: PCWSTR) -> usize;
}
