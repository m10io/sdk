//! Types related to individual account indexes within an account tree or account ID.

use super::{error::*, raw::*};
use std::convert::TryFrom;

/// Root issuance account index.
///
/// Root issuance accounts are always assigned a single byte index, limiting the number of root
/// issuance accounts within a single ledger to 256.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RootAccountIndex(pub(super) RawAccountIndex);

impl RootAccountIndex {
    /// Attempts to create a new root issuance account index from the given value.
    ///
    /// # Errors
    ///
    /// Returns [`AccountIdError::IndexRange`] if the index exceeds the supported range for root
    /// issuance accounts.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::RootAccountIndex;
    ///
    /// let index = RootAccountIndex::new(0xff)?;
    /// assert_eq!(index.as_raw(), 255);
    ///
    /// let index_res = RootAccountIndex::new(0x100);
    /// assert!(index_res.is_err());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`AccountIdError::IndexRange`]: ../error/enum.AccountIdError.html#variant.IndexRange
    #[inline]
    pub fn new(raw: RawAccountIndex) -> Result<Self, AccountIdError> {
        if (raw & !ROOT_ACCOUNT_INDEX_MASK) == 0 {
            Ok(Self(raw))
        } else {
            Err(AccountIdError::IndexRange)
        }
    }

    /// Creates a new root issuance account index from the given value without checking for
    /// out-of-range index values.
    ///
    /// # Safety
    ///
    /// The caller must ensure any values provided fit within the supported root account index
    /// range. Exceeding this range can lead to undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::RootAccountIndex;
    ///
    /// let index = unsafe { RootAccountIndex::new_unchecked(1) };
    /// assert_eq!(index.as_raw(), 1);
    /// ```
    pub const unsafe fn new_unchecked(raw: RawAccountIndex) -> Self {
        Self(raw)
    }

    /// Returns the raw integer value of a root issuance account index.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::RootAccountIndex;
    ///
    /// let index = RootAccountIndex::new(1)?;
    /// assert_eq!(index.as_raw(), 1);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub const fn as_raw(self) -> RawAccountIndex {
        self.0
    }
}

impl TryFrom<RawAccountIndex> for RootAccountIndex {
    type Error = AccountIdError;

    #[inline]
    fn try_from(raw: RawAccountIndex) -> Result<Self, Self::Error> {
        Self::new(raw)
    }
}

impl From<RootAccountIndex> for RawAccountIndex {
    #[inline]
    fn from(index: RootAccountIndex) -> Self {
        index.as_raw()
    }
}

impl_fmt_traits!(RootAccountIndex);

/// Non-root issuance account index.
///
/// Non-root issuance accounts are assigned a 23-bit index, limiting the number of child issuance
/// accounts of a given issuance account (root or non-root) to 8,388,608.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IssuanceAccountIndex(pub(super) RawAccountIndex);

impl IssuanceAccountIndex {
    /// Attempts to create a new non-root issuance account index from the given value.
    ///
    /// # Errors
    ///
    /// Returns [`AccountIdError::IndexRange`] if the index exceeds the supported range for non-root
    /// issuance accounts.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::IssuanceAccountIndex;
    ///
    /// let index = IssuanceAccountIndex::new(0x7f_ffff)?;
    /// assert_eq!(index.as_raw(), 8_388_607);
    ///
    /// let index_res = IssuanceAccountIndex::new(0x80_ffff);
    /// assert!(index_res.is_err());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`AccountIdError::IndexRange`]: ../error/enum.AccountIdError.html#variant.IndexRange
    #[inline]
    pub fn new(raw: RawAccountIndex) -> Result<Self, AccountIdError> {
        if (raw & !ISSUANCE_ACCOUNT_INDEX_MASK) == 0 {
            Ok(Self(raw))
        } else {
            Err(AccountIdError::IndexRange)
        }
    }

    /// Creates a new non-root issuance account index from the given value without checking for
    /// out-of-range index values.
    ///
    /// # Safety
    ///
    /// The caller must ensure any values provided fit within the supported issuance account index
    /// range. Exceeding this range can lead to undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::IssuanceAccountIndex;
    ///
    /// let index = unsafe { IssuanceAccountIndex::new_unchecked(1) };
    /// assert_eq!(index.as_raw(), 1);
    /// ```
    pub const unsafe fn new_unchecked(raw: RawAccountIndex) -> Self {
        Self(raw)
    }

    /// Returns the raw integer value for a non-root issuance account index.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::IssuanceAccountIndex;
    ///
    /// let index = IssuanceAccountIndex::new(1)?;
    /// assert_eq!(index.as_raw(), 1);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn as_raw(self) -> RawAccountIndex {
        self.0
    }

    pub fn next(self) -> Self {
        Self(self.0.checked_add(1).unwrap())
    }
}

impl TryFrom<RawAccountIndex> for IssuanceAccountIndex {
    type Error = AccountIdError;

    #[inline]
    fn try_from(raw: RawAccountIndex) -> Result<Self, Self::Error> {
        Self::new(raw)
    }
}

impl From<IssuanceAccountIndex> for RawAccountIndex {
    #[inline]
    fn from(index: IssuanceAccountIndex) -> Self {
        index.as_raw()
    }
}

impl_fmt_traits!(IssuanceAccountIndex);

/// Leaf account index.
///
/// Leaf accounts are assigned a 39-bit index, limiting the number of child leaf accounts of a given
/// issuance account (root or non-root) to 549,755,813,888.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LeafAccountIndex(pub(super) RawAccountIndex);

impl LeafAccountIndex {
    /// Attempts to create a new leaf account index from the given value.
    ///
    /// # Errors
    ///
    /// Returns [`AccountIdError::IndexRange`] if the index exceeds the supported range for non-root
    /// issuance accounts.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::LeafAccountIndex;
    ///
    /// let index = LeafAccountIndex::new(0x7f_ffff_ffff)?;
    /// assert_eq!(index.as_raw(), 549_755_813_887);
    ///
    /// let index_res = LeafAccountIndex::new(0x80_0000_0000);
    /// assert!(index_res.is_err());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`AccountIdError::IndexRange`]: ../error/enum.AccountIdError.html#variant.IndexRange
    #[inline]
    pub fn new(raw: RawAccountIndex) -> Result<Self, AccountIdError> {
        if (raw & !LEAF_ACCOUNT_INDEX_MASK) == 0 {
            Ok(Self(raw))
        } else {
            Err(AccountIdError::IndexRange)
        }
    }

    /// Creates a new leaf account index from the given value without checking for out-of-range
    /// index values.
    ///
    /// # Safety
    ///
    /// The caller must ensure any values provided fit within the supported leaf account index
    /// range. Exceeding this range can lead to undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::LeafAccountIndex;
    ///
    /// let index = unsafe { LeafAccountIndex::new_unchecked(1) };
    /// assert_eq!(index.as_raw(), 1);
    /// ```
    pub const unsafe fn new_unchecked(raw: RawAccountIndex) -> Self {
        Self(raw)
    }

    /// Returns the raw integer value of a leaf account index.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::LeafAccountIndex;
    ///
    /// let index = LeafAccountIndex::new(1)?;
    /// assert_eq!(index.as_raw(), 1);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn as_raw(self) -> RawAccountIndex {
        self.0
    }
}

impl TryFrom<RawAccountIndex> for LeafAccountIndex {
    type Error = AccountIdError;

    #[inline]
    fn try_from(raw: RawAccountIndex) -> Result<Self, Self::Error> {
        Self::new(raw)
    }
}

impl From<LeafAccountIndex> for RawAccountIndex {
    #[inline]
    fn from(index: LeafAccountIndex) -> Self {
        index.as_raw()
    }
}

impl_fmt_traits!(LeafAccountIndex);

/// Index of an account within a given account subtree.
///
/// Each [`AccountId`] is composed of a number of account indexes indicating the full path to a
/// given account within the account tree. This value wraps the different types of account indices
/// used within the tree.
///
/// [`AccountId`]: ../id/struct.AccountId.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AccountIndex {
    /// Root issuance account index.
    Root(RootAccountIndex),

    /// Non-root issuance account index.
    Issuance(IssuanceAccountIndex),

    /// Leaf account index.
    Leaf(LeafAccountIndex),
}

impl AccountIndex {
    /// Returns the raw integer value of this account index.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{
    ///     AccountIndex, IssuanceAccountIndex, LeafAccountIndex, RootAccountIndex,
    /// };
    ///
    /// let index = AccountIndex::Root(RootAccountIndex::new(5)?);
    /// assert_eq!(index.as_raw(), 5);
    ///
    /// let index = AccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert_eq!(index.as_raw(), 6);
    ///
    /// let index = AccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert_eq!(index.as_raw(), 7);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_raw(self) -> RawAccountIndex {
        match self {
            Self::Root(index) => index.as_raw(),
            Self::Issuance(index) => index.as_raw(),
            Self::Leaf(index) => index.as_raw(),
        }
    }

    /// Returns the wrapped [`RootAccountIndex`] if this is a root account index or `None`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{
    ///     AccountIndex, IssuanceAccountIndex, LeafAccountIndex, RootAccountIndex,
    /// };
    ///
    /// let index = AccountIndex::Root(RootAccountIndex::new(5)?);
    /// assert_eq!(index.as_root_account_index(), Some(RootAccountIndex::new(5)?));
    ///
    /// let index = AccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert_eq!(index.as_root_account_index(), None);
    ///
    /// let index = AccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert_eq!(index.as_root_account_index(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`RootAccountIndex`]: struct.RootAccountIndex.html
    pub fn as_root_account_index(self) -> Option<RootAccountIndex> {
        match self {
            Self::Root(index) => Some(index),
            Self::Issuance(_) | Self::Leaf(_) => None,
        }
    }

    /// Returns the wrapped [`IssuanceAccountIndex`] if this is a non-root issuance account index or
    /// `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{
    ///     AccountIndex, IssuanceAccountIndex, LeafAccountIndex, RootAccountIndex,
    /// };
    ///
    /// let index = AccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert_eq!(index.as_issuance_account_index(), Some(IssuanceAccountIndex::new(6)?));
    ///
    /// let index = AccountIndex::Root(RootAccountIndex::new(5)?);
    /// assert_eq!(index.as_issuance_account_index(), None);
    ///
    /// let index = AccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert_eq!(index.as_issuance_account_index(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`IssuanceAccountIndex`]: struct.IssuanceAccountIndex.html
    pub fn as_issuance_account_index(self) -> Option<IssuanceAccountIndex> {
        match self {
            Self::Issuance(index) => Some(index),
            Self::Root(_) | Self::Leaf(_) => None,
        }
    }

    /// Returns the wrapped [`LeafAccountIndex`] if this is a leaf account index or `None`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{
    ///     AccountIndex, IssuanceAccountIndex, LeafAccountIndex, RootAccountIndex,
    /// };
    ///
    /// let index = AccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert_eq!(index.as_leaf_account_index(), Some(LeafAccountIndex::new(7)?));
    ///
    /// let index = AccountIndex::Root(RootAccountIndex::new(5)?);
    /// assert_eq!(index.as_leaf_account_index(), None);
    ///
    /// let index = AccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert_eq!(index.as_leaf_account_index(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`LeafAccountIndex`]: struct.LeafAccountIndex.html
    pub fn as_leaf_account_index(self) -> Option<LeafAccountIndex> {
        match self {
            Self::Leaf(index) => Some(index),
            Self::Root(_) | Self::Issuance(_) => None,
        }
    }

    /// Returns the wrapped [`IssuanceAccountIndex`] or [`LeafAccountIndex`] as a
    /// [`NonRootAccountIndex`] if this is a non-root account index or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{
    ///     AccountIndex, IssuanceAccountIndex, LeafAccountIndex, NonRootAccountIndex,
    ///     RootAccountIndex,
    /// };
    ///
    /// let index = AccountIndex::Root(RootAccountIndex::new(5)?);
    /// assert_eq!(index.as_non_root_account_index(), None);
    ///
    /// let index = AccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert_eq!(
    ///     index.as_non_root_account_index(),
    ///     Some(NonRootAccountIndex::Issuance(IssuanceAccountIndex::new(6)?))
    /// );
    ///
    /// let index = AccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert_eq!(
    ///     index.as_non_root_account_index(),
    ///     Some(NonRootAccountIndex::Leaf(LeafAccountIndex::new(7)?))
    /// );
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`IssuanceAccountIndex`]: struct.IssuanceAccountIndex.html
    /// [`LeafAccountIndex`]: struct.LeafAccountIndex.html
    /// [`NonRootAccountIndex`]: enum.NonRootAccountIndex.html
    pub fn as_non_root_account_index(self) -> Option<NonRootAccountIndex> {
        match self {
            Self::Root(_) => None,
            Self::Issuance(index) => Some(NonRootAccountIndex::Issuance(index)),
            Self::Leaf(index) => Some(NonRootAccountIndex::Leaf(index)),
        }
    }

    /// Returns whether this an [`Root`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{
    ///     AccountIndex, IssuanceAccountIndex, LeafAccountIndex, RootAccountIndex,
    /// };
    ///
    /// let index = AccountIndex::Root(RootAccountIndex::new(5)?);
    /// assert!(index.is_root());
    ///
    /// let index = AccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert!(!index.is_root());
    ///
    /// let index = AccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert!(!index.is_root());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Root`]: #variant.Root
    pub fn is_root(self) -> bool {
        match self {
            Self::Root(_) => true,
            Self::Issuance(_) | Self::Leaf(_) => false,
        }
    }

    /// Returns whether this an [`Issuance`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{
    ///     AccountIndex, IssuanceAccountIndex, LeafAccountIndex, RootAccountIndex,
    /// };
    ///
    /// let index = AccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert!(index.is_issuance());
    ///
    /// let index = AccountIndex::Root(RootAccountIndex::new(5)?);
    /// assert!(!index.is_issuance());
    ///
    /// let index = AccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert!(!index.is_issuance());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Issuance`]: #variant.Issuance
    pub fn is_issuance(self) -> bool {
        match self {
            Self::Root(_) | Self::Leaf(_) => false,
            Self::Issuance(_) => true,
        }
    }

    /// Returns whether this a [`Leaf`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{
    ///     AccountIndex, IssuanceAccountIndex, LeafAccountIndex, RootAccountIndex,
    /// };
    ///
    /// let index = AccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert!(index.is_leaf());
    ///
    /// let index = AccountIndex::Root(RootAccountIndex::new(5)?);
    /// assert!(!index.is_leaf());
    ///
    /// let index = AccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert!(!index.is_leaf());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Leaf`]: #variant.Leaf
    pub fn is_leaf(self) -> bool {
        match self {
            Self::Root(_) | Self::Issuance(_) => false,
            Self::Leaf(_) => true,
        }
    }
}

impl From<RootAccountIndex> for AccountIndex {
    #[inline]
    fn from(index: RootAccountIndex) -> Self {
        Self::Root(index)
    }
}

impl From<IssuanceAccountIndex> for AccountIndex {
    #[inline]
    fn from(index: IssuanceAccountIndex) -> Self {
        Self::Issuance(index)
    }
}

impl From<LeafAccountIndex> for AccountIndex {
    #[inline]
    fn from(index: LeafAccountIndex) -> Self {
        Self::Leaf(index)
    }
}

impl From<NonRootAccountIndex> for AccountIndex {
    #[inline]
    fn from(index: NonRootAccountIndex) -> Self {
        match index {
            NonRootAccountIndex::Issuance(index) => Self::Issuance(index),
            NonRootAccountIndex::Leaf(index) => Self::Leaf(index),
        }
    }
}

impl TryFrom<AccountIndex> for RootAccountIndex {
    type Error = TryFromAccountIndexError;

    fn try_from(index: AccountIndex) -> Result<Self, Self::Error> {
        index
            .as_root_account_index()
            .ok_or(TryFromAccountIndexError)
    }
}

impl TryFrom<AccountIndex> for IssuanceAccountIndex {
    type Error = TryFromAccountIndexError;

    fn try_from(index: AccountIndex) -> Result<Self, Self::Error> {
        index
            .as_issuance_account_index()
            .ok_or(TryFromAccountIndexError)
    }
}

impl TryFrom<AccountIndex> for LeafAccountIndex {
    type Error = TryFromAccountIndexError;

    fn try_from(index: AccountIndex) -> Result<Self, Self::Error> {
        index
            .as_leaf_account_index()
            .ok_or(TryFromAccountIndexError)
    }
}

impl TryFrom<AccountIndex> for NonRootAccountIndex {
    type Error = TryFromAccountIndexError;

    fn try_from(index: AccountIndex) -> Result<Self, Self::Error> {
        index
            .as_non_root_account_index()
            .ok_or(TryFromAccountIndexError)
    }
}

/// Index of a non-root account within a given account subtree.
///
/// Each [`AccountId`] is composed of a number of account indexes indicating the full path to a
/// given account within the account tree. The first index is always a [`RootAccountIndex`], while
/// subsequent indexes are always either an [`IssuanceAccountIndex`] or [`LeafAccountIndex`]. This
/// value wraps the latter two index types for contexts where either type can be used.
///
/// [`AccountId`]: ../id/struct.AccountId.html
/// [`RootAccountIndex`]: struct.RootAccountIndex.html
/// [`IssuanceAccountIndex`]: struct.IssuanceAccountIndex.html
/// [`LeafAccountIndex`]: struct.LeafAccountIndex.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NonRootAccountIndex {
    /// Non-root issuance account index.
    Issuance(IssuanceAccountIndex),

    /// Leaf account index.
    Leaf(LeafAccountIndex),
}

impl NonRootAccountIndex {
    /// Returns the raw integer value of this account index.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{IssuanceAccountIndex, LeafAccountIndex, NonRootAccountIndex};
    ///
    /// let index = NonRootAccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert_eq!(index.as_raw(), 6);
    ///
    /// let index = NonRootAccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert_eq!(index.as_raw(), 7);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_raw(self) -> RawAccountIndex {
        match self {
            Self::Issuance(index) => index.as_raw(),
            Self::Leaf(index) => index.as_raw(),
        }
    }

    /// Returns the wrapped [`IssuanceAccountIndex`] if this is a non-root issuance account index or
    /// `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{IssuanceAccountIndex, LeafAccountIndex, NonRootAccountIndex};
    ///
    /// let index = NonRootAccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert_eq!(index.as_issuance_account_index(), Some(IssuanceAccountIndex::new(6)?));
    ///
    /// let index = NonRootAccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert_eq!(index.as_issuance_account_index(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`IssuanceAccountIndex`]: struct.IssuanceAccountIndex.html
    pub fn as_issuance_account_index(self) -> Option<IssuanceAccountIndex> {
        match self {
            Self::Issuance(index) => Some(index),
            Self::Leaf(_) => None,
        }
    }

    /// Returns the wrapped [`LeafAccountIndex`] if this is a leaf account index or `None`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{IssuanceAccountIndex, LeafAccountIndex, NonRootAccountIndex};
    ///
    /// let index = NonRootAccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert_eq!(index.as_leaf_account_index(), Some(LeafAccountIndex::new(7)?));
    ///
    /// let index = NonRootAccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert_eq!(index.as_leaf_account_index(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`LeafAccountIndex`]: struct.LeafAccountIndex.html
    pub fn as_leaf_account_index(self) -> Option<LeafAccountIndex> {
        match self {
            Self::Leaf(index) => Some(index),
            Self::Issuance(_) => None,
        }
    }

    /// Returns whether this an [`Issuance`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{IssuanceAccountIndex, LeafAccountIndex, NonRootAccountIndex};
    ///
    /// let index = NonRootAccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert!(index.is_issuance());
    ///
    /// let index = NonRootAccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert!(!index.is_issuance());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Issuance`]: #variant.Issuance
    pub fn is_issuance(self) -> bool {
        match self {
            Self::Leaf(_) => false,
            Self::Issuance(_) => true,
        }
    }

    /// Returns whether this a [`Leaf`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{IssuanceAccountIndex, LeafAccountIndex, NonRootAccountIndex};
    ///
    /// let index = NonRootAccountIndex::Leaf(LeafAccountIndex::new(7)?);
    /// assert!(index.is_leaf());
    ///
    /// let index = NonRootAccountIndex::Issuance(IssuanceAccountIndex::new(6)?);
    /// assert!(!index.is_leaf());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Leaf`]: #variant.Leaf
    pub fn is_leaf(self) -> bool {
        match self {
            Self::Leaf(_) => true,
            Self::Issuance(_) => false,
        }
    }
}

impl From<IssuanceAccountIndex> for NonRootAccountIndex {
    #[inline]
    fn from(index: IssuanceAccountIndex) -> Self {
        Self::Issuance(index)
    }
}

impl From<LeafAccountIndex> for NonRootAccountIndex {
    #[inline]
    fn from(index: LeafAccountIndex) -> Self {
        Self::Leaf(index)
    }
}

impl TryFrom<NonRootAccountIndex> for IssuanceAccountIndex {
    type Error = TryFromAccountIndexError;

    fn try_from(index: NonRootAccountIndex) -> Result<Self, Self::Error> {
        index
            .as_issuance_account_index()
            .ok_or(TryFromAccountIndexError)
    }
}

impl TryFrom<NonRootAccountIndex> for LeafAccountIndex {
    type Error = TryFromAccountIndexError;

    fn try_from(index: NonRootAccountIndex) -> Result<Self, Self::Error> {
        index
            .as_leaf_account_index()
            .ok_or(TryFromAccountIndexError)
    }
}
