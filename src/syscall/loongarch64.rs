// LoongArch has the following registers:
//
// |                      General-purpose Register Convension                       |
// | ============================================================================== |
// | Symbolic Name  | Number          | Usage                                       |
// | ============== | =============== | =========================================== |
// | zero           | 0               | Constant 0                                  |
// | ra             | 1               | Return address                              |
// | tp             | 2               | Thread pointer                              |
// | sp             | 3               | Stack pointer                               |
// | a0 - a1        | 4 - 5           | Argument registers / return value registers |
// | a2 - a7        | 6 - 11          | Argument registers                          |
// | t0 - t8        | 12 - 20         | Tempotaty registers                         |
// | u0             | 21              | Reserved                                    |
// | fp             | 22              | Frame pointer / Static register             |
// | s0 - s8        | 23 - 31         | Static registers                            |
//
// |               Floating-point Register Convention                              |
// | ============================================================================= |
// | Symbolic Name | Number          | Usage                                       |
// | ============= | =============== | =========================================== |
// | fa0 - fa1     | 0 - 1           | Argument registers / return value registers |
// | fa2 - fa7     | 2 - 7           | Argument registers                          |
// | ft0 - ft15    | 8 - 23          | Temporary registers                         |
// | fs0 - fs7     | 24 - 31         | Static registers                            |
//
// Note that v0, v1, fv0, fv1 are just old aliases to a0, a1, fa0, fa1 (not recommended to use)
//
// The following registers are used for args 1-7:
//
// arg1: %a0
// arg2: %a1
// arg3: %a2
// arg4: %a3
// arg5: %a4
// arg6: %a5
// arg7: %a6
//
// %a7 is the syscall number
// %a0, %a1 is the return value
// registers t0 - t8 should be clobbered

use core::arch::asm;

/// Issues a raw system call with 0 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall0(n: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall 0",
        in("$a7") n,
        lateout("$a0") ret,
        // All temporary registers are always clobbered
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw system call with 1 argument.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall1(n: usize, arg1: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall 0",
        in("$a7") n,
        inlateout("$a0") arg1 => ret,
        // All temporary registers are always clobbered
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw system call with 2 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall2(n: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall 0",
        in("$a7") n,
        inlateout("$a0") arg1 => ret,
        in("$a1") arg2,
        // All temporary registers are always clobbered
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw system call with 3 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall3(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "syscall 0",
        in("$a7") n,
        inlateout("$a0") arg1 => ret,
        in("$a1") arg2,
        in("$a2") arg3,
        // All temporary registers are always clobbered
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw system call with 4 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall4(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "syscall 0",
        in("$a7") n,
        inlateout("$a0") arg1 => ret,
        in("$a1") arg2,
        in("$a2") arg3,
        in("$a3") arg4,
        // All temporary registers are always clobbered
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw system call with 5 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall5(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "syscall 0",
        in("$a7") n,
        inlateout("$a0") arg1 => ret,
        in("$a1") arg2,
        in("$a2") arg3,
        in("$a3") arg4,
        in("$a4") arg5,
        // All temporary registers are always clobbered
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw system call with 6 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall6(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "syscall 0",
        in("$a7") n,
        inlateout("$a0") arg1 => ret,
        in("$a1") arg2,
        in("$a2") arg3,
        in("$a3") arg4,
        in("$a4") arg5,
        in("$a5") arg6,
        // All temporary registers are always clobbered
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}
