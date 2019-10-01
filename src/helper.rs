use crate::raw::*;
use crate::*;

fn syscall_ret(ret: i64) -> Result<i64, i64> {
    if ret as u64 >= -4096i64 as u64 {
        Err(-ret)
    } else {
        Ok(ret)
    }
}

#[inline]
pub fn syscall0(no: SyscallNo) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, 0, 0, 0, 0, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall1(no: SyscallNo, a0: i64) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, 0, 0, 0, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall2(no: SyscallNo, a0: i64, a1: i64) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, 0, 0, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall3(no: SyscallNo, a0: i64, a1: i64, a2: i64) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, a2, 0, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall4(
    no: SyscallNo,
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, a2, a3, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall5(
    no: SyscallNo,
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
    a4: i64,
) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, a2, a3, a4, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall6(
    no: SyscallNo,
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
    a4: i64,
    a5: i64,
) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, a2, a3, a4, a5) };
    syscall_ret(r)
}

