use crate::nr::SyscallNo;

extern "C" {
    fn internal_syscall0(nr: i64) -> i64;
    fn internal_syscall1(nr: i64, arg1: u64) -> i64;
    fn internal_syscall2(nr: i64, arg1: u64, arg2: u64) -> i64;
    fn internal_syscall3(nr: i64, arg1: u64, arg2: u64, arg3: u64) -> i64;
    fn internal_syscall4(
        nr: i64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
    ) -> i64;
    fn internal_syscall5(
        nr: i64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
    ) -> i64;
    fn internal_syscall6(
        nr: i64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> i64;
}

#[inline(always)]
pub(crate) fn syscall_ret(ret: i64) -> Result<i64, i64> {
    if ret as u64 >= -4096i64 as u64 {
        Err(-ret)
    } else {
        Ok(ret)
    }
}

#[inline(always)]
#[doc(hidden)]
pub unsafe fn syscall0(nr: SyscallNo) -> Result<i64, i64> {
    syscall_ret(internal_syscall0(nr as i64))
}

#[inline(always)]
#[doc(hidden)]
pub unsafe fn syscall1(nr: SyscallNo, a1: u64) -> Result<i64, i64> {
    syscall_ret(internal_syscall1(nr as i64, a1))
}

#[inline(always)]
#[doc(hidden)]
pub unsafe fn syscall2(nr: SyscallNo, a1: u64, a2: u64) -> Result<i64, i64> {
    syscall_ret(internal_syscall2(nr as i64, a1, a2))
}

#[inline(always)]
#[doc(hidden)]
pub unsafe fn syscall3(
    nr: SyscallNo,
    a1: u64,
    a2: u64,
    a3: u64,
) -> Result<i64, i64> {
    syscall_ret(internal_syscall3(nr as i64, a1, a2, a3))
}

#[inline(always)]
#[doc(hidden)]
pub unsafe fn syscall4(
    nr: SyscallNo,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
) -> Result<i64, i64> {
    syscall_ret(internal_syscall4(nr as i64, a1, a2, a3, a4))
}

#[inline(always)]
#[doc(hidden)]
pub unsafe fn syscall5(
    nr: SyscallNo,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> Result<i64, i64> {
    syscall_ret(internal_syscall5(nr as i64, a1, a2, a3, a4, a5))
}

#[inline(always)]
#[doc(hidden)]
pub unsafe fn syscall6(
    nr: SyscallNo,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
) -> Result<i64, i64> {
    syscall_ret(internal_syscall6(nr as i64, a1, a2, a3, a4, a5, a6))
}
