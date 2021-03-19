/*
 * This was derived from `musl/arch/x86_64/syscall_arch.h`.
 */

long __syscall0(long nr) {
    unsigned long int ret;
    asm volatile (
        "syscall"
        : "=a" (ret)
        : "0" (nr)
        : "memory", "cc", "r11", "cx");
    return (long) ret;
}

long __syscall1(long nr, unsigned long a1) {
    unsigned long int ret;
    register unsigned long _a1 asm ("rdi") = a1;
    asm volatile (
        "syscall"
        : "=a" (ret)
        : "0" (nr), "r" (_a1)
        : "memory", "cc", "r11", "cx");
    return (long) ret;
}

long __syscall2(long nr, unsigned long a1, unsigned long a2) {
    unsigned long int ret;
    register unsigned long _a1 asm ("rdi") = a1;
    register unsigned long _a2 asm ("rsi") = a2;
    asm volatile (
        "syscall"
        : "=a" (ret)
        : "0" (nr), "r" (_a1), "r" (_a2)
        : "memory", "cc", "r11", "cx");
    return (long) ret;
}

long __syscall3(long nr, unsigned long a1, unsigned long a2, unsigned long a3) {
    unsigned long int ret;
    register unsigned long _a1 asm ("rdi") = a1;
    register unsigned long _a2 asm ("rsi") = a2;
    register unsigned long _a3 asm ("rdx") = a3;
    asm volatile (
        "syscall"
        : "=a" (ret)
        : "0" (nr), "r" (_a1), "r" (_a2), "r" (_a3)
        : "memory", "cc", "r11", "cx");
    return (long) ret;
}

long __syscall4(long nr, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4) {
    unsigned long int ret;
    register unsigned long _a1 asm ("rdi") = a1;
    register unsigned long _a2 asm ("rsi") = a2;
    register unsigned long _a3 asm ("rdx") = a3;
    register unsigned long _a4 asm ("r10") = a4;
    asm volatile (
        "syscall"
        : "=a" (ret)
        : "0" (nr), "r" (_a1), "r" (_a2), "r" (_a3), "r"(_a4)
        : "memory", "cc", "r11", "cx");
    return (long) ret;
}

long __syscall5(long nr, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5) {
    unsigned long int ret;
    register unsigned long _a1 asm ("rdi") = a1;
    register unsigned long _a2 asm ("rsi") = a2;
    register unsigned long _a3 asm ("rdx") = a3;
    register unsigned long _a4 asm ("r10") = a4;
    register unsigned long _a5 asm ("r8")  = a5;
    asm volatile (
        "syscall"
        : "=a" (ret)
        : "0" (nr), "r" (_a1), "r" (_a2), "r" (_a3), "r"(_a4), "r"(_a5)
        : "memory", "cc", "r11", "cx");
    return (long) ret;
}

long __syscall6(long nr, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5, unsigned long a6) {
    unsigned long int ret;
    register unsigned long _a1 asm ("rdi") = a1;
    register unsigned long _a2 asm ("rsi") = a2;
    register unsigned long _a3 asm ("rdx") = a3;
    register unsigned long _a4 asm ("r10") = a4;
    register unsigned long _a5 asm ("r8")  = a5;
    register unsigned long _a6 asm ("r9")  = a6;
    asm volatile (
        "syscall"
        : "=a" (ret)
        : "0" (nr), "r" (_a1), "r" (_a2), "r" (_a3), "r"(_a4), "r"(_a5), "r"(_a6)
        : "memory", "cc", "r11", "cx");
    return (long) ret;
}

