//! define SyscallArgs and SyscallRet
//! provide helper functions/trait impls to pack/unpack
//! SyscallArgs and SyscallRet.
//! io:Error is not implemented for better no_std support.

use core::result::Result;

/// The 6 arguments of a syscall, raw untyped version.
///
/// TODO: Use a helper function to convert to a structured Syscall+Args enum.
#[derive(PartialEq, Debug, Eq, Clone, Copy)]
pub struct SyscallArgs {
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
}

impl SyscallArgs {
    pub fn new(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> Self {
        SyscallArgs {
            arg0: a0,
            arg1: a1,
            arg2: a2,
            arg3: a3,
            arg4: a4,
            arg5: a5,
        }
    }
}

impl From<&[u64; 6]> for SyscallArgs {
    fn from(args: &[u64; 6]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: args[3],
            arg4: args[4],
            arg5: args[5],
        }
    }
}

impl From<&[u64; 5]> for SyscallArgs {
    fn from(args: &[u64; 5]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: args[3],
            arg4: args[4],
            arg5: 0,
        }
    }
}

impl From<&[u64; 4]> for SyscallArgs {
    fn from(args: &[u64; 4]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: args[3],
            arg4: 0,
            arg5: 0,
        }
    }
}

impl From<&[u64; 3]> for SyscallArgs {
    fn from(args: &[u64; 3]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: 0,
            arg4: 0,
            arg5: 0,
        }
    }
}

impl From<&[u64; 2]> for SyscallArgs {
    fn from(args: &[u64; 2]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: 0,
            arg3: 0,
            arg4: 0,
            arg5: 0,
        }
    }
}

impl From<&[u64; 1]> for SyscallArgs {
    fn from(args: &[u64; 1]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: 0,
            arg2: 0,
            arg3: 0,
            arg4: 0,
            arg5: 0,
        }
    }
}

impl From<&[u64; 0]> for SyscallArgs {
    fn from(_args: &[u64; 0]) -> Self {
        SyscallArgs {
            arg0: 0,
            arg1: 0,
            arg2: 0,
            arg3: 0,
            arg4: 0,
            arg5: 0,
        }
    }
}

#[macro_export]
macro_rules! syscall_args {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        $crate::args::SyscallArgs::new($a, $b, $c, $d, $e, $f)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        $crate::args::SyscallArgs::new($a, $b, $c, $d, $e, 0)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::args::SyscallArgs::new($a, $b, $c, $d, 0, 0)
    };
    ($a:expr, $b:expr, $c:expr) => {
        $crate::args::SyscallArgs::new($a, $b, $c, 0, 0, 0)
    };
    ($a:expr, $b:expr) => {
        $crate::args::SyscallArgs::new($a, $b, 0, 0, 0, 0)
    };
    ($a:expr) => {
        $crate::args::SyscallArgs::new($a, 0, 0, 0, 0, 0)
    };
    () => {
        $crate::args::SyscallArgs::new(0, 0, 0, 0, 0, 0)
    };
}

#[test]
fn syscall_args_macro_test() {
    assert_eq!(
        syscall_args!(1, 2, 3, 4, 5, 6),
        SyscallArgs::new(1, 2, 3, 4, 5, 6)
    );
    assert_eq!(
        syscall_args!(1, 2, 3, 4, 5),
        SyscallArgs::new(1, 2, 3, 4, 5, 0)
    );
    assert_eq!(
        syscall_args!(1, 2, 3, 4),
        SyscallArgs::new(1, 2, 3, 4, 0, 0)
    );
    assert_eq!(syscall_args!(1, 2, 3), SyscallArgs::new(1, 2, 3, 0, 0, 0));
    assert_eq!(syscall_args!(1, 2), SyscallArgs::new(1, 2, 0, 0, 0, 0));
    assert_eq!(syscall_args!(1), SyscallArgs::new(1, 0, 0, 0, 0, 0));
    assert_eq!(syscall_args!(), SyscallArgs::new(0, 0, 0, 0, 0, 0));
}

#[test]
fn syscall_args_from_u64_slice() {
    assert_eq!(
        SyscallArgs::from(&[1, 2, 3, 4, 5, 6]),
        syscall_args!(1, 2, 3, 4, 5, 6)
    );
    assert_eq!(
        SyscallArgs::from(&[1, 2, 3, 4, 5]),
        syscall_args!(1, 2, 3, 4, 5)
    );
    assert_eq!(SyscallArgs::from(&[1, 2, 3, 4]), syscall_args!(1, 2, 3, 4));
    assert_eq!(SyscallArgs::from(&[1, 2, 3]), syscall_args!(1, 2, 3));
    assert_eq!(SyscallArgs::from(&[1, 2]), syscall_args!(1, 2));
    assert_eq!(SyscallArgs::from(&[1]), syscall_args!(1));
    assert_eq!(SyscallArgs::from(&[0]), syscall_args!());
}

/// syscall return value
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct SyscallRet {
    pub retval: i64,
}

impl From<i64> for SyscallRet {
    fn from(retval: i64) -> Self {
        SyscallRet { retval }
    }
}

impl From<Result<i64, i64>> for SyscallRet {
    fn from(res: Result<i64, i64>) -> Self {
        match res {
            Ok(ret_ok) => SyscallRet { retval: ret_ok },
            Err(ret_err) => SyscallRet { retval: -ret_err },
        }
    }
}

impl From<Result<i64, i32>> for SyscallRet {
    fn from(res: Result<i64, i32>) -> Self {
        match res {
            Ok(ret_ok) => SyscallRet { retval: ret_ok },
            Err(ret_err) => SyscallRet {
                retval: -ret_err as i64,
            },
        }
    }
}

impl From<SyscallRet> for Result<i64, i64> {
    fn from(res: SyscallRet) -> Result<i64, i64> {
        crate::helper::syscall_ret(res.retval)
    }
}

impl From<SyscallRet> for Result<i64, i32> {
    fn from(res: SyscallRet) -> Result<i64, i32> {
        crate::helper::syscall_ret(res.retval).map_err(|e| e as i32)
    }
}

#[test]
fn syscall_ret_err_test() {
    let ok_value = 0x7fff_1234_5678i64;
    let err_value = 22i64;
    let ok1: Result<i64, i64> = Ok(ok_value);
    assert_eq!(SyscallRet::from(ok1), SyscallRet::from(ok_value));

    let err1: Result<i64, i64> = Err(err_value);
    assert_eq!(SyscallRet::from(err1), SyscallRet::from(-err_value as i64));

    let ok2: Result<i64, i32> = Ok(ok_value);
    assert_eq!(SyscallRet::from(ok2), SyscallRet::from(ok_value));

    let err2: Result<i64, i32> = Err(err_value as i32);
    assert_eq!(SyscallRet::from(err2), SyscallRet::from(-err_value as i64));

    let res1: SyscallRet = ok1.into();
    assert_eq!(res1, SyscallRet::from(ok_value));

    let res2: SyscallRet = err1.into();
    assert_eq!(res2, SyscallRet::from(-err_value as i64));

    let ok1_1: Result<i64, i64> = res1.into();
    assert_eq!(ok1_1, ok1);

    let err1_1: Result<i64, i64> = res2.into();
    assert_eq!(err1_1, err1);
}
