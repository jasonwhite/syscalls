use crate::Errno;
use crate::Sysno;

extern "C" {
    fn __syscall0(nr: usize) -> usize;
    fn __syscall1(nr: usize, arg1: usize) -> usize;
    fn __syscall2(nr: usize, arg1: usize, arg2: usize) -> usize;
    fn __syscall3(nr: usize, arg1: usize, arg2: usize, arg3: usize) -> usize;
    fn __syscall4(
        nr: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    ) -> usize;
    fn __syscall5(
        nr: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> usize;
    fn __syscall6(
        nr: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        arg6: usize,
    ) -> usize;
}

/// Issues a system call with 0 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall0(nr: Sysno) -> Result<usize, Errno> {
    Errno::from_ret(__syscall0(nr as usize))
}

/// Issues a system call with 1 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall1(nr: Sysno, a1: usize) -> Result<usize, Errno> {
    Errno::from_ret(__syscall1(nr as usize, a1))
}

/// Issues a system call with 2 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline(always)]
pub unsafe fn syscall2(
    nr: Sysno,
    a1: usize,
    a2: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(__syscall2(nr as usize, a1, a2))
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
    a1: usize,
    a2: usize,
    a3: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(__syscall3(nr as usize, a1, a2, a3))
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
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(__syscall4(nr as usize, a1, a2, a3, a4))
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
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(__syscall5(nr as usize, a1, a2, a3, a4, a5))
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
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(__syscall6(nr as usize, a1, a2, a3, a4, a5, a6))
}
