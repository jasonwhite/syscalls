use syscalls::*;

#[test]
fn test_syscall() {
    let s = "Hello\0";
    assert_eq!(unsafe { syscall!(SYS_write, 1, s.as_ptr() as *const _, 6) }, Ok(6));
}
