#[macro_use]
mod macros;

#[cfg(target_arch = "arm")]
pub mod arm;
#[cfg(target_arch = "mips")]
pub mod mips;
#[cfg(target_arch = "mips64")]
pub mod mips64;
#[cfg(target_arch = "powerpc")]
pub mod powerpc;
#[cfg(target_arch = "powerpc64")]
pub mod powerpc64;
#[cfg(target_arch = "s390x")]
pub mod s390x;
#[cfg(target_arch = "sparc")]
pub mod sparc;
#[cfg(target_arch = "sparc64")]
pub mod sparc64;
#[cfg(target_arch = "x86")]
pub mod x86;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(target_arch = "mips")]
pub use mips::*;

#[cfg(target_arch = "mips64")]
pub use mips64::*;

#[cfg(target_arch = "powerpc")]
pub use powerpc::*;

#[cfg(target_arch = "powerpc64")]
pub use powerpc64::*;

#[cfg(target_arch = "s390x")]
pub use s390x::*;

#[cfg(target_arch = "sparc")]
pub use sparc::*;

#[cfg(target_arch = "sparc64")]
pub use sparc64::*;

#[cfg(target_arch = "x86")]
pub use x86::*;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
