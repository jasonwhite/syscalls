use crate::nr::*;
use crate::raw::*;
use crate::*;

fn syscall_ret(ret: i64) -> Result<i64, i64> {
    if ret as u64 >= -4096i64 as u64 {
        Err(-ret)
    } else {
        Ok(ret)
    }
}

#[inline]
pub fn syscall0(no: SyscallNo) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, 0, 0, 0, 0, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall1(no: SyscallNo, a0: i64) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, 0, 0, 0, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall2(no: SyscallNo, a0: i64, a1: i64) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, 0, 0, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall3(no: SyscallNo, a0: i64, a1: i64, a2: i64) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, a2, 0, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall4(
    no: SyscallNo,
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, a2, a3, 0, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall5(
    no: SyscallNo,
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
    a4: i64,
) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, a2, a3, a4, 0) };
    syscall_ret(r)
}

#[inline]
pub fn syscall6(
    no: SyscallNo,
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
    a4: i64,
    a5: i64,
) -> Result<i64, i64> {
    let r = unsafe { untraced_syscall(no as i32, a0, a1, a2, a3, a4, a5) };
    syscall_ret(r)
}

pub fn __mmap(
    addr: *mut (),
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i64,
) -> Result<*mut (), i64> {
    syscall!(
        SYS_mmap,
        addr as i64,
        length as i64,
        i64::from(prot),
        i64::from(flags),
        i64::from(fd),
        i64::from(offset)
    )
    .map(|x| x as *mut _)
}

pub fn __munmap(ptr: *mut (), size: usize) -> Result<i32, i64> {
    syscall!(SYS_munmap, ptr as i64, size as i64, 0, 0, 0, 0).map(|x| x as i32)
}

pub fn __mremap(
    old_addr: *mut (),
    old_size: usize,
    new_size: usize,
    flags: i32,
) -> Result<i32, i64> {
    syscall!(
        SYS_mremap,
        old_addr as i64,
        old_size as i64,
        new_size as i64,
        i64::from(flags)
    )
    .map(|x| x as i32)
}

pub fn __mprotect(addr: *mut (), len: usize, prot: i32) -> Result<(), i64> {
    syscall!(SYS_mprotect, addr as i64, len as i64, i64::from(prot)).map(|_| ())
}

pub fn __madvise(addr: *mut (), len: usize, advise: i32) -> Result<(), i64> {
    syscall!(SYS_madvise, addr, len, i64::from(advise)).map(|_| ())
}
