//! Enables the creation of an array-backed O(1) map of a syscall to any type
//! `T`.
//!
//! # Examples
//!
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

type DataArray<T> = [MaybeUninit<T>; Sysno::table_size()];

/// A map of syscalls.
///
/// This provides constant-time lookup of syscalls within a bitset.
pub struct SysnoMap<T> {
    is_set: SysnoSet,
    data: DataArray<T>,
}

/// Get internal data index based on sysno value
#[inline(always)]
const fn get_idx(sysno: Sysno) -> usize {
    (sysno.id() as usize) - (Sysno::first().id() as usize)
}

impl<T> SysnoMap<T> {
    /// Initialize an empty syscall map, must have hardcoded size `Sysno::table_size()`.
    pub const fn new() -> Self {
        Self {
            is_set: SysnoSet::empty(),
            data: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    /// Returns true if the map contains the given syscall.
    pub const fn contains_key(&self, sysno: Sysno) -> bool {
        self.is_set.contains(sysno)
    }

    /// Clears the map, removing all syscalls.
    pub fn clear(&mut self) {
        for sysno in self.is_set.iter() {
            unsafe { self.data[get_idx(sysno)].assume_init_drop() }
        }
        self.is_set.clear();
    }

    /// Returns true if the map is empty. Athough this is an O(1) operation
    /// (because the total number of syscalls is always constant), it must
    /// always iterate over the whole map to determine if it is empty or not.
    /// Thus, this may have a large, constant overhead.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_set.is_empty()
    }

    /// Returns the number of syscalls in the map. Although This is an O(1)
    /// operation (because the total number of syscalls is always constant), it
    /// must always iterate over the whole map to determine how many items it
    /// has. Thus, this may have a large, constant overhead.
    #[inline]
    pub fn count(&self) -> usize {
        self.is_set.count()
    }

    /// Inserts the given syscall into the map. Returns true if the syscall was
    /// not already in the map.
    pub fn insert(&mut self, sysno: Sysno, value: T) -> Option<T> {
        let uninit = &mut self.data[get_idx(sysno)];
        if self.is_set.insert(sysno) {
            // Was not already in the set.
            uninit.write(value);
            None
        } else {
            // Was already in the set.
            let old = core::mem::replace(uninit, MaybeUninit::new(value));
            Some(unsafe { old.assume_init() })
        }
    }

    /// Removes the given syscall from the map. Returns old value if the syscall
    /// was in the map.
    pub fn remove(&mut self, sysno: Sysno) -> Option<T> {
        if self.is_set.remove(sysno) {
            let old = core::mem::replace(
                &mut self.data[get_idx(sysno)],
                MaybeUninit::uninit(),
            );
            Some(unsafe { old.assume_init() })
        } else {
            None
        }
    }

    /// Returns a reference to the value corresponding to `sysno`. Returns
    /// `None` if the syscall is not in the map.
    #[inline]
    pub fn get(&self, sysno: Sysno) -> Option<&T> {
        if self.is_set.contains(sysno) {
            Some(unsafe { self.data[get_idx(sysno)].assume_init_ref() })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value corresponding to `sysno`.
    /// Returns `None` if the syscall is not in the map.
    #[inline]
    pub fn get_mut(&mut self, sysno: Sysno) -> Option<&mut T> {
        if self.is_set.contains(sysno) {
            Some(unsafe { self.data[get_idx(sysno)].assume_init_mut() })
        } else {
            None
        }
    }

    /// Returns an iterator that iterates over the syscalls contained in the map.
    pub fn iter(&self) -> SysnoMapIter<T> {
        SysnoMapIter {
            iter: self.is_set.iter(),
            data: &self.data,
        }
    }

    /// Returns an iterator that iterates over all enabled values contained in
    /// the map.
    pub fn values(&self) -> SysnoMapValues<T> {
        SysnoMapValues(self.is_set.iter(), &self.data)
    }
}

impl<T: Copy> SysnoMap<T> {
    /// Initialize a syscall map from the given slice. Note that `T` must be
    /// `Copy` due to `const fn` limitations.
    ///
    /// This is useful for constructing a static callback table.
    ///
    /// # Example
    ///
    /// ```
    /// use syscalls::{Sysno, SysnoMap};
    ///
    /// static CALLBACKS: SysnoMap<fn() -> i32> = SysnoMap::from_slice(&[
    ///     (Sysno::openat, || 42),
    ///     (Sysno::close, || 43),
    /// ]);
    ///
    /// static DESCRIPTIONS: SysnoMap<&'static str> = SysnoMap::from_slice(&[
    ///     (Sysno::openat, "open and possibly create a file"),
    ///     (Sysno::close, "close a file descriptor"),
    /// ]);
    ///
    /// assert_eq!(CALLBACKS[Sysno::openat](), 42);
    /// assert_eq!(DESCRIPTIONS[Sysno::close], "close a file descriptor");
    /// ```
    pub const fn from_slice(slice: &[(Sysno, T)]) -> Self {
        let mut data: DataArray<T> =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut is_set = SysnoSet::empty();

        // Use while-loop because for-loops are not yet allowed in const-fns.
        // https://github.com/rust-lang/rust/issues/87575
        let mut i = 0;
        while i < slice.len() {
            let sysno = slice[i].0;
            let (idx, mask) = SysnoSet::get_idx_mask(sysno);
            is_set.data[idx] |= mask;
            data[get_idx(sysno)] = MaybeUninit::new(slice[i].1);
            i += 1;
        }

        Self { is_set, data }
    }
}

impl<T> Drop for SysnoMap<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

/// An iterator over the syscall (number, value) pairs contained in a
/// [`SysnoMap`].
pub struct SysnoMapIter<'a, T> {
    iter: SysnoSetIter<'a>,
    data: &'a DataArray<T>,
}

impl<'a, T> Iterator for SysnoMapIter<'a, T> {
    type Item = (Sysno, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|sysno| {
            let value = unsafe { self.data[get_idx(sysno)].assume_init_ref() };
            (sysno, value)
        })
    }
}

/// An iterator over the syscall values contained in a [`SysnoMap`].
pub struct SysnoMapValues<'a, T>(SysnoSetIter<'a>, &'a DataArray<T>);

impl<'a, T> Iterator for SysnoMapValues<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|sysno| unsafe { self.1[get_idx(sysno)].assume_init_ref() })
    }
}

impl<T: fmt::Debug> fmt::Debug for SysnoMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.is_set.iter().map(|sysno| {
                (sysno, unsafe {
                    self.data[get_idx(sysno)].assume_init_ref()
                })
            }))
            .finish()
    }
}

impl<T> Extend<(Sysno, T)> for SysnoMap<T> {
    fn extend<I: IntoIterator<Item = (Sysno, T)>>(&mut self, iter: I) {
        for (sysno, value) in iter {
            self.insert(sysno, value);
        }
    }
}

impl<T> core::ops::Index<Sysno> for SysnoMap<T> {
    type Output = T;

    fn index(&self, sysno: Sysno) -> &T {
        self.get(sysno).expect("no entry found for key")
    }
}

impl<T> core::ops::IndexMut<Sysno> for SysnoMap<T> {
    fn index_mut(&mut self, sysno: Sysno) -> &mut T {
        self.get_mut(sysno).expect("no entry found for key")
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
        let mut map = SysnoMap::<fn() -> i32>::new();
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
        // The order of the debug output is not guaranteed, so we can't do an
        // exact match.
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
