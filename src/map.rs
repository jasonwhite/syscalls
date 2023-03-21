//! Enables the creation of a syscall hashmap.

use super::Sysno;
use crate::SysnoSet;
use core::fmt;
use std::fmt::Debug;
use std::mem;

/// A macro to simplify creation and optionally initializing a syscall map.
///
/// # Examples
///
/// Create an empty map with a default value of `0` and initialize it with some values:
/// ```
/// # use syscalls::{syscall_map, Sysno};
/// let mut map = syscall_map!(
///     0;
///     Sysno::open => 0,
///     Sysno::close => 42,
/// );
/// ```
///
/// Create a new map with the defaults set for a set of syscalls:
/// ```
/// # use syscalls::{syscall_map, Sysno, SysnoSet};
/// let mut map = syscall_map!(0, SysnoSet::all());
/// assert_eq!(map.count(), SysnoSet::all().count());
/// ```
///
/// Use custom copy-iable type:
/// ```
/// # use syscalls::{syscall_map, Sysno};
/// #[derive(Copy, Clone, Default)]
/// struct Point { x: i32, y: i32 }
/// let mut map = syscall_map!(Point::default());
/// map.insert(Sysno::open, Point { x: 1, y: 2 });
/// ```
///
/// Use function callbacks:
/// ```
/// # use syscalls::{syscall_map, Sysno, SysnoMap};
/// type Handler = fn() -> i32;
/// let map: SysnoMap<Option<Handler>> = syscall_map!(
///     None::<Handler>;
///     Sysno::open => Some(|| 1),
///     Sysno::close => Some(|| -1),
/// );
///
/// assert_eq!(map.get(Sysno::open).unwrap().unwrap()(), 1);
/// assert_eq!(map.get(Sysno::close).unwrap().unwrap()(), -1);
/// ```
#[macro_export]
macro_rules! syscall_map {
    ($default:expr) => {
        $crate::SysnoMap::new([$default; $crate::Sysno::table_size()])
    };
    ($default:expr, $syscall_set:expr) => {{
        $crate::SysnoMap::new_with_set([$default; $crate::Sysno::table_size()], $syscall_set)
    }};
    ($default:expr; $($sysno:expr),* $(,)?) => {{
        let mut map = $crate::syscall_map!($default);
        $(
            map.insert($sysno, $default);
        )*
        map
    }};
    ($default:expr; $($sysno:expr => $value:expr),* $(,)?) => {{
        let mut map = $crate::syscall_map!($default);
        $(
            map.insert($sysno, $value);
        )*
        map
    }};
}

/// A map of syscalls.
///
/// This provides constant-time lookup of syscalls within a bitset.
///
/// # Examples
///
/// ```
/// # use syscalls::{syscall_map, Sysno, SysnoMap};
/// let mut syscalls = syscall_map!(
///     0;
///     Sysno::open => 0,
///     Sysno::close => 42,
/// );
/// assert!(!syscalls.is_empty());
/// assert_eq!(syscalls.remove(Sysno::open), Some(0));
/// assert_eq!(syscalls.insert(Sysno::close, 4), Some(42));
/// assert!(syscalls.contains_key(Sysno::close));
/// assert_eq!(syscalls.get(Sysno::close), Some(&4));
/// assert_eq!(syscalls.insert(Sysno::close, 11), Some(4));
/// assert_eq!(syscalls.count(), 1);
/// assert_eq!(syscalls.remove(Sysno::close), Some(11));
/// assert!(syscalls.is_empty());
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct SysnoMap<T> {
    is_set: SysnoSet,
    data: [T; Sysno::table_size()],
}

impl<T: Default> SysnoMap<T> {
    /// Initialize an empty syscall map, must have hardcoded size `Sysno::table_size()`.
    pub fn new(data: [T; Sysno::table_size()]) -> Self {
        Self {
            is_set: SysnoSet::empty(),
            data,
        }
    }

    /// Initialize an syscall map for the given syscalls.
    pub fn new_with_set(
        data: [T; Sysno::table_size()],
        sysno_set: SysnoSet,
    ) -> Self {
        Self {
            is_set: sysno_set,
            data,
        }
    }

    /// Returns true if the map contains the given syscall.
    pub const fn contains_key(&self, sysno: Sysno) -> bool {
        self.is_set.contains(sysno)
    }

    /// Clears the map, removing all syscalls.
    pub fn clear(&mut self) {
        self.is_set.clear();
        for elem in self.data.iter_mut() {
            *elem = T::default();
        }
    }

    /// Returns true if the map is empty. This is an O(n) operation as
    /// it must iterate over the entire bitset.
    pub fn is_empty(&self) -> bool {
        self.is_set.is_empty()
    }

    /// Returns the number of syscalls in the map. This is an O(n) operation as
    /// it must count the number of bits in the bitset.
    pub fn count(&self) -> usize {
        self.is_set.count()
    }

    /// Inserts the given syscall into the map. Returns true if the syscall was not already in the map.
    pub fn insert(&mut self, sysno: Sysno, value: T) -> Option<T> {
        let old = mem::replace(&mut self.data[data_idx(sysno)], value);
        if self.is_set.insert(sysno) {
            None
        } else {
            Some(old)
        }
    }

    /// Removes the given syscall from the map. Returns true if the syscall was in the map.
    pub fn remove(&mut self, sysno: Sysno) -> Option<T> {
        if self.is_set.remove(sysno) {
            Some(mem::take(&mut self.data[data_idx(sysno)]))
        } else {
            None
        }
    }

    #[inline]
    pub fn get(&self, sysno: Sysno) -> Option<&T> {
        if self.is_set.contains(sysno) {
            Some(&self.data[data_idx(sysno)])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, sysno: Sysno) -> Option<&mut T> {
        if self.is_set.contains(sysno) {
            Some(&mut self.data[data_idx(sysno)])
        } else {
            None
        }
    }

    /// Returns an iterator that iterates over the syscalls contained in the map.
    pub fn iter(&self) -> impl Iterator<Item = (Sysno, &T)> {
        self.is_set
            .iter()
            .map(move |sysno| (sysno, &self.data[data_idx(sysno)]))
    }

    /// Returns an iterator that iterates over all enabled values contained in the map.
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.is_set
            .iter()
            .map(move |sysno| &self.data[data_idx(sysno)])
    }
}

/// Get internal data index based on sysno value
#[inline(always)]
const fn data_idx(sysno: Sysno) -> usize {
    (sysno.id() as usize) - (Sysno::first().id() as usize)
}

impl<T: Debug> Debug for SysnoMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(
                self.is_set
                    .iter()
                    .map(|sysno| (sysno, &self.data[data_idx(sysno)])),
            )
            .finish()
    }
}

impl<T: Default> Extend<(Sysno, T)> for SysnoMap<T> {
    fn extend<I: IntoIterator<Item = (Sysno, T)>>(&mut self, iter: I) {
        for (sysno, value) in iter {
            self.insert(sysno, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(syscall_map!(0_u8).count(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut map = syscall_map!(0_u8);
        assert!(map.is_empty());
        assert_eq!(map.insert(Sysno::openat, 42), None);
        assert!(!map.is_empty());
        assert_eq!(map.get(Sysno::openat), Some(&42));
        map.remove(Sysno::openat);
        assert!(map.is_empty());
        assert_eq!(map.get(Sysno::openat), None);
    }

    #[test]
    fn test_count() {
        let mut map = syscall_map!(0_u8);
        assert_eq!(map.count(), 0);
        assert_eq!(map.insert(Sysno::openat, 42), None);
        assert_eq!(map.count(), 1);
        assert_eq!(map.insert(Sysno::first(), 4), None);
        assert_eq!(map.count(), 2);
        assert_eq!(map.insert(Sysno::last(), 5), None);
        assert_eq!(map.count(), 3);
        assert_eq!(map.values().sum::<u8>(), 51);
    }

    #[test]
    fn test_insert_remove() {
        let mut map = syscall_map!(0);
        assert_eq!(map.insert(Sysno::openat, 42), None);
        assert!(map.contains_key(Sysno::openat));
        assert_eq!(map.count(), 1);

        assert_eq!(map.insert(Sysno::openat, 4), Some(42));
        assert!(map.contains_key(Sysno::openat));
        assert_eq!(map.count(), 1);

        assert_eq!(map.remove(Sysno::openat), Some(4));
        assert!(!map.contains_key(Sysno::openat));
        assert_eq!(map.count(), 0);

        assert_eq!(map.remove(Sysno::openat), None);
    }

    #[test]
    fn test_debug() {
        let map = syscall_map!(0; Sysno::read => 42, Sysno::openat => 10);
        let result = format!("{:?}", map);
        // The order of the debug output is not guaranteed, so we can't do an exact match
        assert_eq!(result.len(), "{read: 42, openat: 10}".len());
        assert!(result.starts_with('{'));
        assert!(result.ends_with('}'));
        assert!(result.contains(", "));
        assert!(result.contains("read: 42"));
        assert!(result.contains("openat: 10"));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_iter() {
        let map = syscall_map!(0; Sysno::read => 42, Sysno::openat => 10);
        assert_eq!(map.iter().collect::<Vec<_>>().len(), 2);
    }
}
