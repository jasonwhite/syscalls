#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    // These architectures require nightly to use inline assembly.
    // See https://github.com/rust-lang/rust/issues/93335
    any(
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "s390x",
        target_arch = "powerpc",
        target_arch = "powerpc65",
    ),
    feature(asm_experimental_arch)
)]

#[macro_use]
mod macros;

mod arch;
mod args;
mod errno;
mod ffi;

pub use arch::*;
pub use args::SyscallArgs;
pub use errno::{Errno, ErrnoSentinel};
pub use ffi::{
    syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6,
};

/// Does a raw syscall.
///
/// # Arguments
///  - `nr`: The syscall number.
///  - `args`: packed arguments
///
/// # Returns
///  - `Ok` on success,
///  - `Err` when the syscall failed (with errno).
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
pub unsafe fn syscall(nr: Sysno, args: &SyscallArgs) -> Result<usize, Errno> {
    syscall6(
        nr, args.arg0, args.arg1, args.arg2, args.arg3, args.arg4, args.arg5,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_syscall1_syscall4() {
        let pmaps = CString::new("/dev/zero").unwrap();
        let fd = unsafe {
            let at_fdcwd = -100isize;
            syscall!(SYS_openat, at_fdcwd, pmaps.as_ptr(), 0)
        }
        .unwrap();

        let mut buffer1: [u8; 64] = unsafe { std::mem::zeroed() };
        let mut buffer2: [u8; 64] = unsafe { std::mem::zeroed() };

        let r1 =
            unsafe { libc::read(fd as i32, buffer1.as_mut_ptr() as _, 64) };

        let s1 = unsafe {
            std::slice::from_raw_parts(
                buffer1.as_mut_ptr() as *const u8,
                r1 as usize,
            )
        };
        let r2 = unsafe { syscall!(SYS_read, fd, buffer2.as_mut_ptr(), 64) };
        let s2 = unsafe {
            std::slice::from_raw_parts(
                buffer1.as_mut_ptr() as *const u8,
                r2.unwrap_or(0) as usize,
            )
        };

        assert_eq!(r2, Ok(r1 as usize));
        assert_eq!(s1, s2);

        let closed = unsafe { syscall!(SYS_close, fd) };
        assert!(closed.is_ok());
    }

    #[test]
    fn test_syscall1_syscall4_2() {
        let pmaps = CString::new("/dev/zero").unwrap();
        let fd = unsafe {
            let at_fdcwd = -100isize;
            syscall!(SYS_openat, at_fdcwd, pmaps.as_ptr(), 0)
        }
        .unwrap();

        let mut buffer1: [u8; 64] = unsafe { std::mem::zeroed() };
        let mut buffer2: [u8; 64] = unsafe { std::mem::zeroed() };

        let args =
            SyscallArgs::from(&[fd as usize, buffer1.as_mut_ptr() as _, 64]);
        let r1 = unsafe { syscall(SYS_read, &args) }.expect("SYS_read failed");

        let s1 = unsafe {
            std::slice::from_raw_parts(
                buffer1.as_mut_ptr() as *const u8,
                r1 as usize,
            )
        };
        let r2 = unsafe { syscall!(SYS_read, fd, buffer2.as_mut_ptr(), 64) };
        let s2 = unsafe {
            std::slice::from_raw_parts(
                buffer1.as_mut_ptr() as *const u8,
                r2.unwrap_or(0) as usize,
            )
        };

        assert_eq!(r2, Ok(r1 as usize));
        assert_eq!(s1, s2);

        let closed = unsafe { syscall!(SYS_close, fd) };
        assert!(closed.is_ok());
    }

    #[test]
    fn test_name() {
        assert_eq!(SYS_write.name(), "write");
        assert_eq!(SYS_fsopen.name(), "fsopen");
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_syscallno() {
        assert_eq!(Sysno::from(2), SYS_open);
        assert_eq!(Sysno::new(2), Some(SYS_open));
        assert_eq!(Sysno::new(-1i32 as usize), None);
        assert_eq!(Sysno::new(1024), None);
    }

    #[test]
    fn test_first() {
        #[cfg(target_arch = "x86_64")]
        assert_eq!(Sysno::first(), Sysno::read);

        #[cfg(target_arch = "x86")]
        assert_eq!(Sysno::first(), Sysno::restart_syscall);
    }
}
