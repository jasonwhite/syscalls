#![cfg_attr(not(feature = "use_std"), no_std)]

#[macro_use]

pub mod helper;
pub mod macros;

// Include the generated system calls.
include!(concat!(env!("OUT_DIR"), "/nr.rs"));

pub use self::helper::*;
pub use SyscallNo::*;

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
}
