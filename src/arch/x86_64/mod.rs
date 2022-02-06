//! Syscalls for the x86_64 architecture.

mod syscall;
mod syscalls;

pub(crate) use self::syscall::*;
pub use self::syscalls::*;
