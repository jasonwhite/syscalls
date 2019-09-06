#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]

pub mod helper;
pub mod macros;
pub mod raw;

// Include the generated system calls.
include!(concat!(env!("OUT_DIR"), "/nr.rs"));

pub use self::helper::*;
pub use self::raw::*;

pub use SyscallNo::*;
