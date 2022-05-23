# Changelog

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
