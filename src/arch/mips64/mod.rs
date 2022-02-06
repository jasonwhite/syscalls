//! Syscalls for the mips64 architecture.

mod syscall;
mod syscalls;

pub(crate) use self::syscall::*;
pub use self::syscalls::*;
