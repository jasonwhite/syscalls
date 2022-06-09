//! Syscalls for the s390x architecture.

mod syscall;
mod syscalls;

pub use self::syscall::*;
pub use self::syscalls::*;
