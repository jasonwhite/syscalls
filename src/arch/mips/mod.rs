//! Syscalls for the mips architecture.

mod syscall;
mod syscalls;

pub(crate) use self::syscall::*;
pub use self::syscalls::*;
