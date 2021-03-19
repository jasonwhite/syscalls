use crate::Errno;
use crate::Sysno;

extern "C" {
    fn __syscall0(nr: i64) -> i64;
    fn __syscall1(nr: i64, arg1: u64) -> i64;
    fn __syscall2(nr: i64, arg1: u64, arg2: u64) -> i64;
    fn __syscall3(nr: i64, arg1: u64, arg2: u64, arg3: u64) -> i64;
    fn __syscall4(nr: i64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> i64;
    fn __syscall5(
        nr: i64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
    ) -> i64;
    fn __syscall6(
        nr: i64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> i64;
}

/// Issues a system call with 0 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall0(nr: Sysno) -> Result<i64, Errno> {
    Errno::from_ret(__syscall0(nr as i64))
}

/// Issues a system call with 1 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall1(nr: Sysno, a1: u64) -> Result<i64, Errno> {
    Errno::from_ret(__syscall1(nr as i64, a1))
}

/// Issues a system call with 2 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
#[inline(always)]
pub unsafe fn syscall2(nr: Sysno, a1: u64, a2: u64) -> Result<i64, Errno> {
    Errno::from_ret(__syscall2(nr as i64, a1, a2))
}

/// Issues a system call with 3 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall3(
    nr: Sysno,
    a1: u64,
    a2: u64,
    a3: u64,
) -> Result<i64, Errno> {
    Errno::from_ret(__syscall3(nr as i64, a1, a2, a3))
}

/// Issues a system call with 4 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall4(
    nr: Sysno,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
) -> Result<i64, Errno> {
    Errno::from_ret(__syscall4(nr as i64, a1, a2, a3, a4))
}

/// Issues a system call with 5 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall5(
    nr: Sysno,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> Result<i64, Errno> {
    Errno::from_ret(__syscall5(nr as i64, a1, a2, a3, a4, a5))
}

/// Issues a system call with 6 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall6(
    nr: Sysno,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
) -> Result<i64, Errno> {
    Errno::from_ret(__syscall6(nr as i64, a1, a2, a3, a4, a5, a6))
}
