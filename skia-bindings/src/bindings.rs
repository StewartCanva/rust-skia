#![allow(clippy::all)]
// https://github.com/rust-lang/rust-bindgen/issues/1651
#![allow(unknown_lints)]
#![allow(deref_nullptr)]
// GrVkBackendContext contains u128 fields on macOS
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
