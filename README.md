# syscalls

This crate automatically generates a list of system calls using the Linux
`<sys/syscall.h>` header.

## `no_std` support
support of `no_std` can be enabled by turn off default features:
```
syscalls = { version = "0.2", default-features = false }
```
