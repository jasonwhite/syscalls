// On s390x, the following registers are used for args 1-6:
// arg1: %r2
// arg2: %r3
// arg3: %r4
// arg4: %r5
// arg5: %r6
// arg6: %r7
//
// syscall number: %r1
// return value: %r2
//
// No other registers are clobbered. syscalls can also modify memory. With the
// `asm!()` macro, it is assumed that memory is clobbered unless the nomem
// option is specified.
use core::arch::asm;

use super::syscalls::Sysno;

/// A syscall that takes 0 arguments.
#[inline(always)]
pub unsafe fn syscall0(n: Sysno) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        out("r2") ret,
        in("r1") n as usize,
    );
    ret
}

/// A syscall that takes 1 argument.
#[inline(always)]
pub unsafe fn syscall1(n: Sysno, arg1: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        lateout("r2") ret,
        in("r1") n as usize,
        in("r2") arg1,
    );
    ret
}

/// A syscall that takes 2 arguments.
#[inline(always)]
pub unsafe fn syscall2(n: Sysno, arg1: usize, arg2: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        lateout("r2") ret,
        in("r1") n as usize,
        in("r2") arg1,
        in("r3") arg2,
    );
    ret
}

/// A syscall that takes 3 arguments.
#[inline(always)]
pub unsafe fn syscall3(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        lateout("r2") ret,
        in("r1") n as usize,
        in("r2") arg1,
        in("r3") arg2,
        in("r4") arg3,
    );
    ret
}

/// A syscall that takes 4 arguments.
#[inline(always)]
pub unsafe fn syscall4(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        lateout("r2") ret,
        in("r1") n as usize,
        in("r2") arg1,
        in("r3") arg2,
        in("r4") arg3,
        in("r5") arg4,
    );
    ret
}

/// A syscall that takes 5 arguments.
#[inline(always)]
pub unsafe fn syscall5(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        lateout("r2") ret,
        in("r1") n as usize,
        in("r2") arg1,
        in("r3") arg2,
        in("r4") arg3,
        in("r5") arg4,
        in("r6") arg5,
    );
    ret
}

/// A syscall that takes 6 arguments.
#[inline(always)]
pub unsafe fn syscall6(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        lateout("r2") ret,
        in("r1") n as usize,
        in("r2") arg1,
        in("r3") arg2,
        in("r4") arg3,
        in("r5") arg4,
        in("r6") arg5,
        in("r7") arg6,
    );
    ret
}
