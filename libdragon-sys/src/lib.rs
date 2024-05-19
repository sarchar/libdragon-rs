#![no_std]
#![feature(alloc_error_handler)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![allow(improper_ctypes)] // TODO getting warnings on u128 types

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

