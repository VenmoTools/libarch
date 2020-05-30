#![no_std]
#![feature(llvm_asm)]
#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![deny(warnings)]
#![allow(unused_doc_comments)]
#![allow(dead_code)]

#[macro_use]
extern crate alloc;

pub mod arch;
pub mod macros;
pub mod result;
