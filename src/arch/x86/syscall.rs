// On x86, the following registers are used for args 1-6:
// arg1: %ebx
// arg2: %ecx
// arg3: %edx
// arg4: %esi
// arg5: %edi
// arg6: %ebp
//
// eax is used for both the syscall number and the syscall return value.
//
// No other registers are clobbered. syscalls can also modify memory. With the
// `asm!()` macro, it is assumed that memory is clobbered unless the nomem
// option is specified.
use core::arch::asm;

use super::syscalls::Sysno;

/// A syscall that takes 0 arguments.
#[inline]
pub unsafe fn syscall0(n: Sysno) -> usize {
    let mut ret: usize;
    asm!(
        "int $0x80",
        inlateout("eax") n as usize => ret,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

/// A syscall that takes 1 argument.
#[inline]
pub unsafe fn syscall1(n: Sysno, arg1: usize) -> usize {
    let mut ret: usize;
    asm!(
        "int $$0x80",
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 2 arguments.
#[inline]
pub unsafe fn syscall2(n: Sysno, arg1: usize, arg2: usize) -> usize {
    let mut ret: usize;
    asm!(
        "int $$0x80",
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 3 arguments.
#[inline]
pub unsafe fn syscall3(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "int $$0x80",
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        in("edx") arg3,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 4 arguments.
#[inline]
pub unsafe fn syscall4(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "xchg esi, {arg4}",
        "int $$0x80",
        "xchg esi, {arg4}",
        // Using esi is not allowed, so we need to use another register to
        // save/restore esi. Thus, we can say that esi is not clobbered.
        arg4 = in(reg) arg4,
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        in("edx") arg3,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 5 arguments.
#[inline]
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
        "xchg esi, {arg4}",
        "int $$0x80",
        "xchg esi, {arg4}",
        // Using esi is not allowed, so we need to use another register to
        // save/restore esi. Thus, we can say that esi is not clobbered.
        arg4 = in(reg) arg4,
        inlateout("eax") n as usize => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        in("edx") arg3,
        in("edi") arg5,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 6 arguments.
#[inline]
pub unsafe fn syscall6(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    // Since using esi and ebp are not allowed and because x86 only has 6
    // general purpose registers (excluding ESP and EBP), we need to push them
    // onto the stack and then set them using a pointer to memory (our input
    // array).
    let mut ret: usize;
    asm!(
        "push ebp",
        "push esi",
        "mov esi, DWORD PTR [eax + 0]", // Set esi to arg4
        "mov ebp, DWORD PTR [eax + 4]", // Set ebp to arg6
        "mov eax, DWORD PTR [eax + 8]", // Lastly, set eax to the syscall number.
        "int $$0x80",
        "pop esi",
        "pop ebp",
        // Set eax to a pointer to our input array.
        inout("eax") &[arg4, arg6, n as usize] => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        in("edx") arg3,
        in("edi") arg5,
        options(preserves_flags)
    );
    ret
}
