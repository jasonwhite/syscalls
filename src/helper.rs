use crate::SyscallNo;

#[inline(always)]
fn syscall_ret(ret: i64) -> Result<i64, i64> {
    if ret as u64 >= -4096i64 as u64 {
        Err(-ret)
    } else {
        Ok(ret)
    }
}

#[inline(always)]
pub unsafe fn syscall0(nr: SyscallNo) -> Result<i64, i64> {
    let ret: i64;
    asm!("syscall" : "={rax}"(ret)
                   : "{eax}"(nr as i32)
                   : "rcx", "r11", "memory"
                   : "volatile");
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall1(nr: SyscallNo, a1: u64) -> Result<i64, i64> {
    let ret: i64;
    asm!("syscall" : "={rax}"(ret)
                   : "{eax}"(nr as i32), "{rdi}"(a1)
                   : "rcx", "r11", "memory"
                   : "volatile");
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall2(nr: SyscallNo, a1: u64, a2: u64) -> Result<i64, i64> {
    let ret: i64;
    asm!("syscall" : "={rax}"(ret)
                   : "{eax}"(nr as i32), "{rdi}"(a1), "{rsi}"(a2)
                   : "rcx", "r11", "memory"
                   : "volatile");
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall3(
    nr: SyscallNo,
    a1: u64,
    a2: u64,
    a3: u64,
) -> Result<i64, i64> {
    let ret: i64;
    asm!("syscall" : "={rax}"(ret)
                   : "{eax}"(nr as i32), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
                   : "rcx", "r11", "memory"
                   : "volatile");
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall4(
    nr: SyscallNo,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
) -> Result<i64, i64> {
    let ret: i64;
    asm!("syscall" : "={rax}"(ret)
                   : "{eax}"(nr as i32), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4)
                   : "rcx", "r11", "memory"
                   : "volatile");
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall5(
    n: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> Result<i64, i64> {
    let ret: i64;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5)
                   : "rcx", "r11", "memory"
                   : "volatile");
    syscall_ret(ret)
}

#[inline(always)]
pub unsafe fn syscall6(
    nr: SyscallNo,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
) -> Result<i64, i64> {
    let ret: i64;
    asm!("syscall" : "={rax}"(ret)
                   : "{eax}"(nr as i32), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
                   : "rcx", "r11", "memory"
                   : "volatile");
    syscall_ret(ret)
}
