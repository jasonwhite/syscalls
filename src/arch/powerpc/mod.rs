//! Syscalls for the powerpc architecture.

mod syscall;
mod syscalls;

pub(crate) use self::syscall::*;
pub use self::syscalls::*;
