// MIPS has the following registers:
//
// | Symbolic Name | Number          | Usage                          |
// | ============= | =============== | ============================== |
// | zero          | 0               | Constant 0.                    |
// | at            | 1               | Reserved for the assembler.    |
// | v0 - v1       | 2 - 3           | Result Registers.              |
// | a0 - a3       | 4 - 7           | Argument Registers 1 ·· · 4.   |
// | t0 - t9       | 8 - 15, 24 - 25 | Temporary Registers 0 · · · 9. |
// | s0 - s7       | 16 - 23         | Saved Registers 0 ·· · 7.      |
// | k0 - k1       | 26 - 27         | Kernel Registers 0 ·· · 1.     |
// | gp            | 28              | Global Data Pointer.           |
// | sp            | 29              | Stack Pointer.                 |
// | fp            | 30              | Frame Pointer.                 |
// | ra            | 31              | Return Address.                |
//
// The following registers are used for args 1-6:
//
// arg1: %a0 ($4)
// arg2: %a1 ($5)
// arg3: %a2 ($6)
// arg4: %a3 ($7)
// arg5: (Passed via user stack)
// arg6: (Passed via user stack)
// arg7: (Passed via user stack)
//
// %v0 is the syscall number.
// %v0 is the return value.
// %a3 is a boolean indicating that an error occurred.
//
//
// All temporary registers are clobbered (8-15, 24-25).
use core::arch::asm;

/// Issues a raw system call with 0 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall0(n: usize) -> usize {
    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") n => ret,
        lateout("$7") err,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}

/// Issues a raw system call with 1 argument.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall1(n: usize, arg1: usize) -> usize {
    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") n => ret,
        lateout("$7") err,
        in("$4") arg1,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}

/// Issues a raw system call with 2 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall2(n: usize, arg1: usize, arg2: usize) -> usize {
    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") n => ret,
        lateout("$7") err,
        in("$4") arg1,
        in("$5") arg2,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
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
    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") n => ret,
        lateout("$7") err,
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
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
    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") n => ret,
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        // $7 is now used for both input and output.
        inlateout("$7") arg4 => err,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
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
    // NOTE: Arg numbers >=5 args get passed via the stack.
    let mut err: usize;
    let mut ret: usize;
    asm!(
        // NOTE: `.set noat` prevents the assembler from warning about using the
        // `%at` (assembler temporary) register. This register could get
        // allocated with `in(reg)` below.
        ".set noat",
        "subu $sp, 32", // Make space on the stack.
        "sw {arg5}, 16($sp)", // Store word arg5 in the stack.
        "syscall",
        "addu $sp, 32", // Restore the stack.
        ".set at",
        arg5 = in(reg) arg5,
        inlateout("$2") n => ret,
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        // $7 is now used for both input and output.
        inlateout("$7") arg4 => err,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
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
    // Things get trickier with >=5 args. arg5 and arg6 are now passed via the
    // stack.
    let mut err: usize;
    let mut ret: usize;
    asm!(
        // NOTE: `.set noat` prevents the assembler from warning about using the
        // `%at` (assembler temporary) register. This register could get
        // allocated with `in(reg)` below.
        ".set noat",
        "subu $sp, 32", // Make space on the stack.
        "sw {arg5}, 16($sp)", // Store word arg5 in the stack.
        "sw {arg6}, 20($sp)", // Store word arg6 in the stack.
        "syscall",
        "addu $sp, 32", // Restore the stack.
        ".set at",
        arg5 = in(reg) arg5,
        arg6 = in(reg) arg6,
        inlateout("$2") n => ret,
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        // $7 is now used for both input and output.
        inlateout("$7") arg4 => err,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}

/// Issues a raw system call with 7 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[allow(unused)]
#[inline]
pub unsafe fn syscall7(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
    arg7: usize,
) -> usize {
    // Things get trickier with >=5 args. arg5 and arg6 are now passed via the
    // stack.
    let mut err: usize;
    let mut ret: usize;
    asm!(
        // NOTE: `.set noat` prevents the assembler from warning about using the
        // `%at` (assembler temporary) register. This register could get
        // allocated with `in(reg)` below.
        ".set noat",
        "subu $sp, 32",       // Make space on the stack.
        "sw {arg5}, 16($sp)", // Store word arg5 in the stack.
        "sw {arg6}, 20($sp)", // Store word arg6 in the stack.
        "sw {arg7}, 24($sp)", // Store word arg6 in the stack.
        "syscall",
        "addu $sp, 32",       // Restore the stack.
        ".set at",
        arg5 = in(reg) arg5,
        arg6 = in(reg) arg6,
        arg7 = in(reg) arg7,
        inlateout("$2") n => ret,
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        // $7 is now used for both input and output.
        inlateout("$7") arg4 => err,
        // All temporary registers are always clobbered
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}
