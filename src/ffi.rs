use crate::Errno;
use crate::Sysno;

use crate::arch;

/// Issues a system call with 0 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall0(nr: Sysno) -> Result<usize, Errno> {
    Errno::from_ret(arch::syscall0(nr))
}

/// Issues a system call with 1 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall1(nr: Sysno, a1: usize) -> Result<usize, Errno> {
    Errno::from_ret(arch::syscall1(nr, a1))
}

/// Issues a system call with 2 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall2(
    nr: Sysno,
    a1: usize,
    a2: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(arch::syscall2(nr, a1, a2))
}

/// Issues a system call with 3 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall3(
    nr: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(arch::syscall3(nr, a1, a2, a3))
}

/// Issues a system call with 4 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall4(
    nr: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(arch::syscall4(nr, a1, a2, a3, a4))
}

/// Issues a system call with 5 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall5(
    nr: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(arch::syscall5(nr, a1, a2, a3, a4, a5))
}

/// Issues a system call with 6 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall6(
    nr: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(arch::syscall6(nr, a1, a2, a3, a4, a5, a6))
}
