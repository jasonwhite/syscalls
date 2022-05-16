/// Performs a syscall.
///
/// Accepts a syscall number and a variable number of arguments (0 to 6).
///
/// # Returns
///  - `Ok` on success, or
///  - `Err(errno)` if the syscall failed.
#[macro_export]
macro_rules! syscall {
    ($nr:expr) => {
        $crate::syscall0($nr)
    };

    ($nr:expr, $a1:expr) => {
        $crate::syscall1($nr, $a1 as usize)
    };

    ($nr:expr, $a1:expr, $a2:expr) => {
        $crate::syscall2($nr, $a1 as usize, $a2 as usize)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::syscall3($nr, $a1 as usize, $a2 as usize, $a3 as usize)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::syscall4(
            $nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::syscall5(
            $nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        $crate::syscall6(
            $nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
            $a6 as usize,
        )
    };
}
