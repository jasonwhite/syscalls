# syscalls

This crate automatically generates a list of system calls using the Linux
`<sys/syscall.h>` header.

## `no_std` support

Support of `no_std` can be enabled by turn off default features:
```
syscalls = { version = "0.3", default-features = false }
```

## Updating the syscall list

Run:
```
cd syscalls-gen
cargo run -- ../src/nr.rs
```
