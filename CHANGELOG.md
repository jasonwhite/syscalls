# Changelog

## v0.6.3

 - Added features to expose the syscall tables of other architectures besides
   the target architecture. There is one feature per architecture and have the
   same name. For example, if the target architecture is `x86-64` and we also
   want the syscall table for `x86`, the `x86` feature can be enabled. Then,
   `syscalls::x86::Sysno` will be exposed.
 - Added the `all` feature, which enables the syscall tables for all
   architectures.
 - Added the `full` feature, which enables all current and future features for
   the crate.
 - Added man page links for all syscalls. Since these are generated, some links
   may be broken.

## v0.6.2

 - Added `SysnoSet` for constructing sets of syscalls. It uses a bitset under
   the hood and provides constant-time lookup and insertion of `Sysno`s.
 - Fixed `Sysno::len()` returning the wrong value for architectures with large
   syscall offsets.
 - Deprecated `Sysno::len()`. Use `Sysno::table_size()` instead. This will be
   removed in the next major version.

## v0.6.1

 - Exposed `syscalls::raw::*` to allow avoidance of the `Result` return type.
   This makes it cleaner to call syscalls like `gettid` that are guaranteed to
   never fail.

## v0.6.0

 - Removed `build.rs` and switched to Rust's inline assembly syntax. This should
   enable better codegen, including the ability to have syscalls get inlined.
 - **Breaking**: Architectures besides `arm`, `x86`, and `x86-64` now require
   nightly.
 - **Breaking**: Removed top-level `SYS_` constants. Just use the `Sysno` enum
   instead.

## v0.5.0

This is a major breaking change from v0.4.

 - Changed all syscalls to take and return `usize` instead of `i64` or `u64`.
   This fixes calling syscalls on 32-bit architectures.
 - Fixed syscall offsets for mips and mips64.
 - Added CI tests for more than just `x86_64`.

## v0.4.2

 - Made `ErrnoSentinel` public.

## v0.4.1

 - Added the ability to invoke syscalls for all architectures except `aarch64`,
   `sparc`, and `sparc64`.
 - Fixed std-dependent Errno trait impls not getting compiled.
 - Made `syscalls::arch::{x86, x86_64, ...}` private.

## v0.4.0

This is a major breaking change from v0.3. You can fix most compilation errors
by simply doing `s/SyscallNo::SYS_/Sysno::/g`.

 - Created this changelog.
 - Renamed `SyscallNo::SYS_*` to `Sysno::*`.
 - Added `Errno` for more Rustic error handling.
 - Changed the `syscalls-gen` script to grab Linux headers from GitHub.
 - Added more architecture support for the syscall table. Issuing syscalls is
   still limited to x86-64, however.
