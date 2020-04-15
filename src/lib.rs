#![cfg_attr(not(feature = "use_std"), no_std)]

#[macro_use]

mod args;
pub mod helper;
pub mod macros;
mod nr;

pub use args::{SyscallArgs, SyscallRet};
pub use helper::{
    syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6,
};
pub use nr::{SyscallNo, SyscallNo::*};

/// do a syscall
/// @nr: syscall number
/// @args: packed arguments
/// @return: Ok on success, Err when syscall failed (with errno)
pub unsafe fn syscall(nr: SyscallNo, args: &SyscallArgs) -> Result<i64, i64> {
    crate::helper::syscall6(
        nr, args.arg0, args.arg1, args.arg2, args.arg3, args.arg4, args.arg5,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_syscall1_syscall4() {
        let pmaps = CString::new("/proc/self/maps").unwrap();
        let fd_ = unsafe {
            let at_fdcwd = (-100i64) as u64;
            syscall!(SYS_openat, at_fdcwd, pmaps.as_ptr(), 0)
        };
        match fd_ {
            Err(_) => {
                assert_eq!("open /proc/self/maps failed", "");
            }
            Ok(fd) => {
                let mut buffer1: [u8; 64] = unsafe { std::mem::zeroed() };
                let mut buffer2: [u8; 64] = unsafe { std::mem::zeroed() };

                let r1 = unsafe {
                    libc::pread64(fd as i32, buffer1.as_mut_ptr() as _, 64, 16)
                };

                let s1 = unsafe {
                    std::slice::from_raw_parts(
                        buffer1.as_mut_ptr() as *const u8,
                        r1 as usize,
                    )
                };
                let r2 = unsafe {
                    syscall!(
                        SYS_pread64,
                        fd,
                        buffer2.as_mut_ptr() as u64,
                        64,
                        16
                    )
                };
                let s2 = unsafe {
                    std::slice::from_raw_parts(
                        buffer1.as_mut_ptr() as *const u8,
                        r2.unwrap_or(0) as usize,
                    )
                };

                assert_eq!(r2, Ok(r1 as i64));
                assert_eq!(s1, s2);

                let closed = unsafe { syscall!(SYS_close, fd as u64) };
                assert!(closed.is_ok());
            }
        }
    }

    #[test]
    fn test_syscall1_syscall4_2() {
        let pmaps = CString::new("/proc/self/maps").unwrap();
        let fd_ = unsafe {
            let at_fdcwd = (-100i64) as u64;
            syscall!(SYS_openat, at_fdcwd, pmaps.as_ptr(), 0)
        };
        match fd_ {
            Err(_) => {
                assert_eq!("open /proc/self/maps failed", "");
            }
            Ok(fd) => {
                let mut buffer1: [u8; 64] = unsafe { std::mem::zeroed() };
                let mut buffer2: [u8; 64] = unsafe { std::mem::zeroed() };

                let args = SyscallArgs::from(&[
                    fd as u64,
                    buffer1.as_mut_ptr() as _,
                    64,
                    16,
                ]);
                let r1 = unsafe { syscall(SYS_pread64, &args) }
                    .expect("SYS_pread64 failed");

                let s1 = unsafe {
                    std::slice::from_raw_parts(
                        buffer1.as_mut_ptr() as *const u8,
                        r1 as usize,
                    )
                };
                let r2 = unsafe {
                    syscall!(
                        SYS_pread64,
                        fd,
                        buffer2.as_mut_ptr() as u64,
                        64,
                        16
                    )
                };
                let s2 = unsafe {
                    std::slice::from_raw_parts(
                        buffer1.as_mut_ptr() as *const u8,
                        r2.unwrap_or(0) as usize,
                    )
                };

                assert_eq!(r2, Ok(r1 as i64));
                assert_eq!(s1, s2);

                let closed = unsafe { syscall!(SYS_close, fd as u64) };
                assert!(closed.is_ok());
            }
        }
    }
}
