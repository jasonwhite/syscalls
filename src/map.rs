//! Enables the creation of an array-backed O(1) map of a syscall to a copyable user type..
//!
//! Create a new map with the defaults set for a set of syscalls:
//! ```
//! # use syscalls::{Sysno, SysnoMap, SysnoSet};
//! let mut map = SysnoMap::init(SysnoSet::all(), 0);
//! assert_eq!(map.count(), SysnoSet::all().count());
//! ```
//!
//! Use custom copy-iable type:
//! ```
//! # use syscalls::{Sysno, SysnoMap};
//! #[derive(Copy, Clone, Default)]
//! struct Point { x: i32, y: i32 }
//!
//! let mut map = SysnoMap::new();
//! map.insert(Sysno::openat, Point { x: 1, y: 2 });
//! assert!(map.get(Sysno::openat).is_some());
//! ```
//!
//! Use function callbacks:
//! ```
//! # use syscalls::{Sysno, SysnoMap};
//! type Handler = fn() -> i32;
//! let mut map = SysnoMap::<Handler>::new();
//! map.insert(Sysno::openat, || 1);
//! map.insert(Sysno::close, || -1);
//! assert_eq!(map.get(Sysno::openat).unwrap()(), 1);
//! assert_eq!(map.get(Sysno::close).unwrap()(), -1);
//! ```
//!
//! ```
//! # use syscalls::{syscall_map, Sysno, SysnoMap};
//! let mut syscalls = syscall_map!(
//!     Sysno::openat => 0,
//!     Sysno::close => 42,
//! );
//! assert!(!syscalls.is_empty());
//! assert_eq!(syscalls.remove(Sysno::openat), Some(0));
//! assert_eq!(syscalls.insert(Sysno::close, 4), Some(42));
//! assert!(syscalls.contains_key(Sysno::close));
//! assert_eq!(syscalls.get(Sysno::close), Some(&4));
//! assert_eq!(syscalls.insert(Sysno::close, 11), Some(4));
//! assert_eq!(syscalls.count(), 1);
//! assert_eq!(syscalls.remove(Sysno::close), Some(11));
//! assert!(syscalls.is_empty());
//! ```

use super::Sysno;
use crate::set::SysnoSetIter;
use crate::SysnoSet;
use core::fmt;
use core::mem::MaybeUninit;

/// A macro to create and initialize a const syscall map => T.
///
/// # Examples
///
/// Create a map with some initialized sysno values:
/// ```
/// # use syscalls::{syscall_map, Sysno};
/// let mut map = syscall_map!(
///     Sysno::openat => 0,
///     Sysno::close => 42,
/// );
/// ```
#[macro_export]
macro_rules! syscall_map {
    ($($sysno:expr => $value:expr),* $(,)?) => {{
        let mut map = $crate::SysnoMap::new();
        $(
            map.insert($sysno, $value);
        )*
        map
    }};
}

type DataArrayType<T> = [MaybeUninit<T>; Sysno::table_size()];

/// A map of syscalls.
///
/// This provides constant-time lookup of syscalls within a bitset.
#[derive(Clone)]
pub struct SysnoMap<T: Copy> {
    is_set: SysnoSet,
    data: DataArrayType<T>,
}

impl<T: Copy> SysnoMap<T> {
    /// Get internal data index based on sysno value
    #[inline(always)]
    const fn get_idx(sysno: Sysno) -> usize {
        (sysno.id() as usize) - (Sysno::first().id() as usize)
    }

    /// Initialize an empty syscall map, must have hardcoded size `Sysno::table_size()`.
    pub const fn new() -> Self {
        Self {
            is_set: SysnoSet::empty(),
            data: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    /// Initialize an syscall map for the given syscalls.
    pub const fn init(sysno_set: SysnoSet, default: T) -> Self {
        let mut data: DataArrayType<T> =
            unsafe { MaybeUninit::uninit().assume_init() };
        let is_set = &sysno_set.data;
        // Use while-loop because for-loops are not yet allowed in const-fns.
        // https://github.com/rust-lang/rust/issues/87575
        let mut opt_id = Some(Sysno::first());
        while let Some(id) = opt_id {
            let (idx, mask) = SysnoSet::get_idx_mask(id);
            if is_set[idx] | mask != 0 {
                data[Self::get_idx(id)] = MaybeUninit::new(default);
            }
            opt_id = id.next();
        }

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
    #[inline]
    pub fn clear(&mut self) {
        for sysno in self.is_set.iter() {
            unsafe { self.data[Self::get_idx(sysno)].assume_init_drop() }
        }
        self.is_set.clear();
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
        let uninit = &mut self.data[Self::get_idx(sysno)];
        if self.is_set.insert(sysno) {
            uninit.write(value);
            None
        } else {
            // TODO: should this be assume_init_read() instead?
            let old = unsafe { uninit.assume_init() };
            uninit.write(value);
            Some(old)
        }
    }

    /// Removes the given syscall from the map. Returns true if the syscall was in the map.
    pub fn remove(&mut self, sysno: Sysno) -> Option<T> {
        let uninit = &mut self.data[Self::get_idx(sysno)];
        if self.is_set.remove(sysno) {
            Some(unsafe { uninit.assume_init() })
        } else {
            None
        }
    }

    #[inline]
    pub fn get(&self, sysno: Sysno) -> Option<&T> {
        if self.is_set.contains(sysno) {
            Some(unsafe { self.data[Self::get_idx(sysno)].assume_init_ref() })
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, sysno: Sysno) -> Option<&mut T> {
        if self.is_set.contains(sysno) {
            Some(unsafe { self.data[Self::get_idx(sysno)].assume_init_mut() })
        } else {
            None
        }
    }

    /// Returns an iterator that iterates over the syscalls contained in the map.
    pub fn iter(&self) -> SysnoMapPairIter<T> {
        SysnoMapPairIter(self.is_set.iter(), &self.data)
    }

    /// Returns an iterator that iterates over all enabled values contained in the map.
    pub fn values(&self) -> SysnoMapValueIter<T> {
        SysnoMapValueIter(self.is_set.iter(), &self.data)
    }
}

impl<T: Copy> Drop for SysnoMap<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

/// An iterator over the syscall (number, value) pairs contained in a [`SysnoMap`].
pub struct SysnoMapPairIter<'a, T: Copy>(
    SysnoSetIter<'a>,
    &'a DataArrayType<T>,
);

impl<'a, T: Copy> Iterator for SysnoMapPairIter<'a, T> {
    type Item = (Sysno, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|sysno| {
            (sysno, unsafe {
                *self.1[SysnoMap::<T>::get_idx(sysno)].assume_init_ref()
            })
        })
    }
}

/// An iterator over the syscall values contained in a [`SysnoMap`].
pub struct SysnoMapValueIter<'a, T: Copy>(
    SysnoSetIter<'a>,
    &'a DataArrayType<T>,
);

impl<'a, T: Copy> Iterator for SysnoMapValueIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|sysno| unsafe {
            *self.1[SysnoMap::<T>::get_idx(sysno)].assume_init_ref()
        })
    }
}

impl<T: fmt::Debug + Copy> fmt::Debug for SysnoMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.is_set.iter().map(|sysno| {
                (sysno, unsafe {
                    self.data[Self::get_idx(sysno)].assume_init_ref()
                })
            }))
            .finish()
    }
}

impl<T: Copy> Extend<(Sysno, T)> for SysnoMap<T> {
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
        assert_eq!(SysnoMap::<u8>::new().count(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut map = SysnoMap::new();
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
        let mut map = SysnoMap::new();
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
    fn test_fn() {
        type Handler = fn() -> i32;
        let mut map = SysnoMap::<Handler>::new();
        map.insert(Sysno::openat, || 1);
        map.insert(Sysno::close, || -1);
        assert_eq!(map.get(Sysno::openat).unwrap()(), 1);
        assert_eq!(map.get(Sysno::close).unwrap()(), -1);
    }

    #[test]
    fn test_fn_macro() {
        type Handler = fn() -> i32;
        let map = syscall_map!(
            Sysno::openat => (|| 1) as Handler,
            Sysno::close => (|| -1) as Handler,
        );
        assert_eq!(map.get(Sysno::openat).unwrap()(), 1);
        assert_eq!(map.get(Sysno::close).unwrap()(), -1);
    }

    #[test]
    fn test_insert_remove() {
        let mut map = SysnoMap::new();
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

    #[cfg(feature = "std")]
    #[test]
    fn test_debug() {
        let map = syscall_map!(Sysno::read => 42, Sysno::openat => 10);
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
        let map = syscall_map!(Sysno::read => 42, Sysno::openat => 10);
        assert_eq!(map.iter().collect::<Vec<_>>().len(), 2);
    }
}
