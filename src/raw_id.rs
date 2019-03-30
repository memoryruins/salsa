use std::fmt;

/// The "raw-id" is used for interned keys in salsa. Typically, it is
/// wrapped in a type of your own devising. For more information about
/// interned keys, see [the interned key RFC][rfc].
///
/// # Creating a `RawId`
//
/// RawId values can be constructed using the `From` impls,
/// which are implemented for `u32` and `usize`:
///
/// ```
/// # use salsa::RawId;
/// let raw_id = RawId::from(22_u32);
/// ```
///
/// You can then convert back to a `u32` like so:
///
/// ```
/// # use salsa::RawId;
/// # let raw_id = RawId::from(22_u32);
/// let value = u32::from(raw_id);
/// assert_eq!(value, 22);
/// ```
///
/// ## Illegal values
///
/// Be warned, however, that `RawId` values cannot be created from
/// *arbitrary* values -- in particular large values greater than
/// `RawId::MAX` will panic. Those large values are reserved so that
/// the Rust compiler can use them as sentinel values, which means
/// that (for example) `Option<RawId>` is represented in a single
/// word.
///
/// ```should_panic
/// # use salsa::RawId;
/// RawId::from(RawId::MAX);
/// ```
///
/// [rfc]: https://github.com/salsa-rs/salsa-rfcs/pull/2
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawId {
    value: u32,
}

impl RawId {
    /// The maximum allowed `RawId`. This value can grow between
    /// releases without affecting semver.
    pub const MAX: u32 = 0xFFFF_FF00;

    unsafe fn new_unchecked(value: u32) -> Self {
        debug_assert!(value < RawId::MAX);
        RawId { value }
    }

    /// Convert this raw-id into a u32 value.
    pub fn as_u32(self) -> u32 {
        self.value
    }

    /// Convert this raw-id into a usize value.
    pub fn as_usize(self) -> usize {
        self.value as usize
    }
}

impl From<RawId> for u32 {
    fn from(raw: RawId) -> u32 {
        raw.value
    }
}

impl From<RawId> for usize {
    fn from(raw: RawId) -> usize {
        raw.value as usize
    }
}

impl From<u32> for RawId {
    fn from(id: u32) -> RawId {
        assert!(id < RawId::MAX);
        unsafe { RawId::new_unchecked(id) }
    }
}

impl From<usize> for RawId {
    fn from(id: usize) -> RawId {
        assert!(id < (RawId::MAX as usize));
        unsafe { RawId::new_unchecked(id as u32) }
    }
}

impl fmt::Debug for RawId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl fmt::Display for RawId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}
