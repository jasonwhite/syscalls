//! Syscalls for the powerpc64 architecture.

mod syscall;
mod syscalls;

pub(crate) use self::syscall::*;
pub use self::syscalls::*;
