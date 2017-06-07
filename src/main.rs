#![allow(
    unused_imports,
)]

#![feature(
    pattern,
    collections_range,
    type_ascription,
    slice_get_slice,
    )]

#[macro_use]
extern crate lazy_static;

mod uv;

// extern crate os_rust;
// use uv;

fn main() {
    uv::ps1::zhttpto();
    println!("Hello, world!");
}
