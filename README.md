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
syscalls = { version = "0.3", default-features = false }
```

### `with-serde`

Various types can be serialized with Serde. This can be enabled with:
```
syscalls = { version = "0.3", features = ["with-serde"] }
```

## Architecture Support

The *Enum* column means that a `Sysno` enum is implemented for this
architecture.

The *Invoke* column means that syscalls can be invoked for this architecture.

|     Arch    | Enum | Invoke |
|:-----------:|:----:|:------:|
|       `arm` |   ✅  |    ✅   |
|   `aarch64` |   ❌  |    ❌   |
|      `mips` |   ✅  |    ✅   |
|    `mips64` |   ✅  |    ✅   |
|   `powerpc` |   ✅  |    ✅   |
| `powerpc64` |   ✅  |    ✅   |
|     `s390x` |   ✅  |    ✅   |
|     `sparc` |   ✅  |    ❌   |
|   `sparc64` |   ✅  |    ❌   |
|       `x86` |   ✅  |    ✅   |
|    `x86_64` |   ✅  |    ✅   |

## Updating the syscall list

Updates are pulled from the `.tbl` files in the Linux source tree.

 1. Change the Linux version in `syscalls-gen/src/main.rs` to the latest
    version. Using a release candidate version is OK.
 2. Run `cd syscalls-gen && cargo run`. This will regenerate the syscall tables
    in `src/arch/`.

