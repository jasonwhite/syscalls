# syscalls

[![Crates.io](https://img.shields.io/crates/v/syscalls?style=for-the-badge)](https://crates.io/crates/syscalls)
[![docs.rs](https://img.shields.io/docsrs/syscalls?style=for-the-badge)](https://docs.rs/syscalls)
![License](https://img.shields.io/crates/l/syscalls.svg?style=for-the-badge)

A list of all Linux system calls.

## Features

 - Provides of all syscalls for multiple architectures.
 - Provides methods for invoking raw syscalls.
 - Provides an `Errno` type for Rustic error handling.

## Feature Flags

### `std`

By default, `std` support is enabled. If you wish to compile in a `no_std`
environment, use:
```
syscalls = { version = "0.5", default-features = false }
```

### `with-serde`

Various types can be serialized with Serde. This can be enabled with:
```
syscalls = { version = "0.5", features = ["with-serde"] }
```

## Architecture Support

The *Enum* column means that a `Sysno` enum is implemented for this
architecture.

The *Invoke* column means that syscalls can be invoked for this architecture.

The *Requires Nightly?* column means that syscalls can only be invoked if
compiled with nightly Rust. This is because inline assembly [is not yet
stabilized for all architectures][asm_experimental_arch].

[asm_experimental_arch]: https://github.com/rust-lang/rust/issues/93335

|     Arch    | Enum  | Invoke  | Requires Nightly? |
|:-----------:|:-----:|:-------:|:-----------------:|
|       `arm` |   ✅  |    ✅   | No ✅             |
|   `aarch64` |   ❌  |    ❌   | N/A               |
|      `mips` |   ✅  |    ✅   | Yes ❌            |
|    `mips64` |   ✅  |    ✅   | Yes ❌            |
|   `powerpc` |   ✅  |    ✅   | Yes ❌            |
| `powerpc64` |   ✅  |    ✅   | Yes ❌            |
|     `s390x` |   ✅  |    ✅   | Yes ❌            |
|     `sparc` |   ✅  |    ❌   | N/A               |
|   `sparc64` |   ✅  |    ❌   | N/A               |
|       `x86` |   ✅  |    ✅   | No ✅             |
|    `x86_64` |   ✅  |    ✅   | No ✅             |

## Updating the syscall list

Updates are pulled from the `.tbl` files in the Linux source tree.

 1. Change the Linux version in `syscalls-gen/src/main.rs` to the latest
    version. Using a release candidate version is OK.
 2. Run `cd syscalls-gen && cargo run`. This will regenerate the syscall tables
    in `src/arch/`.

