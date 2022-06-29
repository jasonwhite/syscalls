/// Helper for generating support code for a list of syscalls.
macro_rules! syscall_enum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $Name:ident {
            $(#[$first_inner:meta])*
            $first_syscall:ident = $first_num:expr,
            $(
                $(#[$inner:meta])*
                $syscall:ident = $num:expr,
            )*
        }

        LAST: $last_syscall:ident;
    ) => {
        /// Complete list of Linux syscalls.
        $(#[$outer])*
        #[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
        #[derive(Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
        #[cfg_attr(feature = "serde_repr", derive(::serde_repr::Serialize_repr, ::serde_repr::Deserialize_repr))]
        #[repr(i32)]
        #[non_exhaustive]
        $vis enum $Name {
            $(#[$first_inner])*
            $first_syscall = $first_num,
            $(
                $(#[$inner])*
                $syscall = $num,
            )*
        }

        impl $Name {
            /// Constructs a new syscall from the given ID. If the ID does not
            /// represent a valid syscall, returns `None`.
            pub const fn new(id: usize) -> Option<Self> {
                match id {
                    $(#[$first_inner])*
                    $first_num => Some(Self::$first_syscall),
                    $(
                        $num => Some(Self::$syscall),
                    )*
                    _ => None,
                }
            }

            /// Returns the name of the syscall.
            pub const fn name(&self) -> &'static str {
                match self {
                    $(#[$first_inner])*
                    Self::$first_syscall => core::stringify!($first_syscall),
                    $(
                        Self::$syscall => core::stringify!($syscall),
                    )*
                }
            }

            /// Returns the next syscall in the table. Returns `None` if this is
            /// the last syscall.
            pub const fn next(&self) -> Option<Self> {
                if let Self::$last_syscall = self {
                    return None;
                }

                let mut next_id = self.id() as usize + 1;

                while next_id < Self::len() {
                    if let Some(next) = Self::new(next_id) {
                        return Some(next);
                    }

                    next_id += 1;
                }

                None
            }

            /// Returns the first syscall in the table.
            pub const fn first() -> Self {
                Self::$first_syscall
            }

            /// Returns the last syscall in the table.
            pub const fn last() -> Self {
                Self::$last_syscall
            }

            /// Returns the syscall number.
            pub const fn id(&self) -> i32 {
                *self as i32
            }

            /// Returns the length of the syscall table, including any gaps.
            pub const fn len() -> usize {
                (Self::last().id() - Self::first().id()) as usize + 1
            }

            /// Returns an iterator that iterates over all possible syscalls.
            pub fn iter() -> impl Iterator<Item = Self> {
                core::iter::successors(Some(Self::first()), |x| x.next())
            }
        }

        impl core::str::FromStr for $Name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(#[$first_inner])*
                    core::stringify!($first_syscall) => Ok(Self::$first_syscall),
                    $(
                        core::stringify!($syscall) => Ok(Self::$syscall),
                    )*
                    _ => Err(()),
                }
            }
        }

        impl core::fmt::Display for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl core::fmt::Debug for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl From<i32> for $Name {
            fn from(id: i32) -> Self {
                Self::new(id as usize)
                    .unwrap_or_else(|| panic!("invalid syscall: {}", id))
            }
        }
    }
}
