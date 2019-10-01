#![cfg_attr(not(feature = "std"), no_std)]
#![feature(asm)]

#[macro_use]

pub mod helper;
pub mod macros;

// Include the generated system calls.
include!(concat!(env!("OUT_DIR"), "/nr.rs"));

pub use self::helper::*;
pub use SyscallNo::*;
