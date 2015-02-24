#![feature(libc, std_misc, unsafe_destructor, core, old_io, collections)]

extern crate libc;

#[allow(non_camel_case_types)]
#[macro_use]
pub mod ffi;

pub mod avcodec;
pub mod avformat;
pub mod avutil;

pub mod util;
