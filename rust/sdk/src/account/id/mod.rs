//! `AccountId` and related types.

use super::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    iter::FusedIterator,
    mem,
};

mod builder;
pub use builder::Builder;

pub mod iter;

mod ext;
pub use ext::AccountIdExt;

/// Packed account ID.
///
/// Account IDs are 16-byte values that reflect the full path to an account entry in the ledger. It
/// is a concatenation of the parent issuance account ID and the index assigned to the account
/// within its parent subtree, as well as a single-byte value indicating the depth of the account
/// within the account tree.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AccountId(RawAccountId);

impl AccountId {
    /// Maximum supported account ID depth value.
    ///
    /// Due to the number of bits reserved for each `AccountId` component, we can only support up to
    /// the following numbers of elements in each ID:
    ///
    /// - The root account (1 byte). Since a root account index is always needed, this is implied
    ///   and not included in the maximum depth value.
    /// - Three issuance accounts (9 bytes total). Each issuance account implies an additional two
    ///   depth levels from the previous issuance account (one depth level is used for leaf accounts
    ///   at each level).
    /// - A final leaf account (5 bytes).
    /// - The depth value itself (1 byte).
    pub const MAX_DEPTH: AccountIdDepth = 7;

    /// Attempts to create a new account ID from a raw integer value, checking the ID for validity.
    ///
    /// # Errors
    ///
    /// Returns one of the following [`AccountIdError`] variants if account ID creation fails:
    ///
    /// - [`InvalidDepth`] if the depth value stored exceeds [`MAX_DEPTH`].
    /// - [`InvalidSpecialBits`] if any non-index bits (i.e. bits reserved for identifying issuance
    ///   account indexes and unused bits within the ID) are set incorrectly.
    ///
    /// # Examples
    ///
    /// Valid account ID:
    /// ```
    /// use m10_sdk::{account_id, account::AccountId};
    ///
    /// let id = AccountId::from_raw(0x0181_2345_8678_9a0f_edcb_a987_0000_0005)?;
    /// assert_eq!(id, account_id![0x01, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Invalid due to unused index bits being set (leaf index value is still present from the
    /// previous example, but we've decreased the depth).
    /// ```
    /// use m10_sdk::account::AccountId;
    ///
    /// let id_res = AccountId::from_raw(0x0181_2345_8678_9a0f_edcb_a987_0000_0004);
    /// assert!(id_res.is_err());
    /// ```
    ///
    /// Invalid due to invalid issuance account bit states.
    /// ```
    /// use m10_sdk::account::AccountId;
    ///
    /// // Leaf account has its issuance bit set.
    /// let id_res = AccountId::from_raw(0x0181_2345_8678_9a8f_edcb_a987_0000_0005);
    /// assert!(id_res.is_err());
    ///
    /// // First issuance account does not have its issuance bit set.
    /// let id_res = AccountId::from_raw(0x0101_2345_8678_9a0f_edcb_a987_0000_0005);
    /// assert!(id_res.is_err());
    /// ```
    ///
    /// Invalid due to depth value greater than [`MAX_DEPTH`]:
    /// ```
    /// use m10_sdk::account::AccountId;
    ///
    /// let id_res = AccountId::from_raw(0x0181_2345_8678_9a8b_cdef_8234_5600_0008);
    /// assert!(id_res.is_err());
    /// ```
    ///
    /// [`AccountIdError`]: ../error/enum.AccountIdError.html
    /// [`InvalidDepth`]: ../error/enum.AccountIdError.html#variant.InvalidDepth
    /// [`MAX_DEPTH`]: #associatedconstant.MAX_DEPTH
    /// [`InvalidSpecialBits`]: ../error/enum.AccountIdError.html#variant.InvalidSpecialBits
    pub fn from_raw(raw: RawAccountId) -> Result<Self, AccountIdError> {
        // Make sure the packed depth value is valid.
        let depth = usize::from((raw as AccountIdDepth) & ACCOUNT_ID_DEPTH_MASK);
        if depth > usize::from(Self::MAX_DEPTH) {
            return Err(AccountIdError::InvalidDepth);
        }

        // Mask out the bits that are expected to be set to specific values (the issuance flag bits
        // and the unused portion of the ID) and compare them with their expected values.
        let issuance_count = depth / 2;
        let leaf_count = depth % 2;

        let chain_bit_count = ROOT_ACCOUNT_INDEX_BIT_COUNT
            + (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1) * issuance_count
            + (LEAF_ACCOUNT_INDEX_BIT_COUNT + 1) * leaf_count;
        let unused_bit_count =
            mem::size_of::<RawAccountId>() * 8 - ACCOUNT_ID_DEPTH_BIT_COUNT - chain_bit_count;
        let unused_mask: RawAccountId = ((1 << unused_bit_count) - 1) << ACCOUNT_ID_DEPTH_BIT_COUNT;

        let mut expected_bits: RawAccountId = 0;
        let mut next_issuance_flag: RawAccountId =
            1 << (mem::size_of::<RawAccountId>() * 8 - ROOT_ACCOUNT_INDEX_BIT_COUNT - 1);
        for _ in 0..issuance_count {
            expected_bits |= next_issuance_flag;
            next_issuance_flag >>= ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1;
        }

        let issuance_flag_mask = if leaf_count != 0 {
            expected_bits | next_issuance_flag
        } else {
            expected_bits
        };

        if raw & (issuance_flag_mask | unused_mask) != expected_bits {
            return Err(AccountIdError::InvalidSpecialBits);
        }

        Ok(Self(raw))
    }

    /// Creates a new account ID from a raw integer value without checking for validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure the depth component of the account ID does not exceed [`MAX_DEPTH`],
    /// and that any bits not used by the stored account index chain or depth component are set to
    /// zero. Creating and using an account ID that violates one of these conditions can lead to
    /// undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::{account_id, account::AccountId};
    ///
    /// let id = unsafe {
    ///     AccountId::from_raw_unchecked(0x0181_2345_8678_9a0f_edcb_a987_0000_0005)
    /// };
    /// assert_eq!(id, account_id![0x01, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`MAX_DEPTH`]: #associatedconstant.MAX_DEPTH
    pub const unsafe fn from_raw_unchecked(raw: RawAccountId) -> Self {
        Self(raw)
    }

    /// Creates a new account ID from a single root account index.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::{account_id, account::{AccountId, RootAccountIndex}};
    ///
    /// let id = AccountId::from_root_account_index(0x37)?;
    /// assert_eq!(id, account_id![0x37]?);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_root_account_index<RootIndexT>(index: RootIndexT) -> Result<Self, AccountIdError>
    where
        RootIndexT: TryInto<RootAccountIndex>,
        AccountIdError: From<RootIndexT::Error>,
    {
        Ok(Self(
            (index.try_into()?.as_raw() as RawAccountId)
                << (mem::size_of::<RawAccountId>() * 8 - ROOT_ACCOUNT_INDEX_BIT_COUNT),
        ))
    }

    /// Returns the raw integer value of this ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account_id;
    ///
    /// let id = account_id![0x01, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// assert_eq!(id.as_raw(), 0x0181_2345_8678_9a0f_edcb_a987_0000_0005);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub const fn as_raw(self) -> RawAccountId {
        self.0
    }

    /// Returns the account depth of this ID.
    ///
    /// Note that depth values start at zero for root accounts.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account_id;
    ///
    /// let id = account_id![0x01, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// assert_eq!(id.depth(), 5);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub const fn depth(self) -> AccountIdDepth {
        (self.0 as AccountIdDepth) & ACCOUNT_ID_DEPTH_MASK
    }

    /// Returns whether this ID represents a leaf account.
    ///
    /// This is the inverse of [`is_issuance`].
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account_id;
    ///
    /// let id = account_id![0x01, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// assert!(id.is_leaf());
    ///
    /// let id = account_id![0x01, 0x01_2345, 0x06_789a]?;
    /// assert!(!id.is_leaf());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`is_issuance`]: #method.is_issuance
    #[inline]
    pub const fn is_leaf(self) -> bool {
        self.0 & 0x1 != 0
    }

    /// Returns whether this ID represents an issuance account (root or non-root).
    ///
    /// This is the inverse of [`is_leaf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account_id;
    ///
    /// let id = account_id![0x01, 0x01_2345, 0x06_789a]?;
    /// assert!(id.is_issuance());
    ///
    /// let id = account_id![0x01, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// assert!(!id.is_issuance());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`is_leaf`]: #method.is_leaf
    #[inline]
    pub const fn is_issuance(self) -> bool {
        self.0 & 0x1 == 0
    }

    /// Returns the root account index component of this ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::{account_id, account::RootAccountIndex};
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// assert_eq!(id.root_account_index(), RootAccountIndex::new(0x42)?);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub const fn root_account_index(self) -> RootAccountIndex {
        let index_offset = mem::size_of::<RawAccountId>() * 8 - ROOT_ACCOUNT_INDEX_BIT_COUNT;
        let raw_index = (self.0 >> index_offset) as RawAccountIndex & ROOT_ACCOUNT_INDEX_MASK;

        RootAccountIndex(raw_index)
    }

    /// Returns an iterator over the non-root issuance account indexes in this account ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::{account_id, account::AccountId};
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// let mut accounts = id.hierarchy();
    /// assert_eq!(accounts.next(), Some(id));
    /// assert_eq!(accounts.next(), id.parent_id());
    /// assert_eq!(accounts.next(), id.parent_id().unwrap().parent_id());
    /// assert_eq!(accounts.next(), id.parent_id().unwrap().parent_id().unwrap().parent_id());
    /// assert_eq!(accounts.next(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn hierarchy(self) -> iter::AccountHierarchyIter {
        iter::AccountHierarchyIter::new(self)
    }

    /// Returns an iterator over the non-root issuance account indexes in this account ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::{account_id, account::IssuanceAccountIndex};
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// let mut issuance_indexes = id.issuance_account_indexes();
    /// assert_eq!(issuance_indexes.next(), Some(IssuanceAccountIndex::new(0x01_2345)?));
    /// assert_eq!(issuance_indexes.next(), Some(IssuanceAccountIndex::new(0x06_789a)?));
    /// assert_eq!(issuance_indexes.next(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn issuance_account_indexes(self) -> iter::IssuanceAccountIndexIter {
        iter::IssuanceAccountIndexIter::new(self)
    }

    /// Returns the leaf account index component of this ID, or `None` if this ID is not a leaf
    /// account ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::{account_id, account::LeafAccountIndex};
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// assert_eq!(id.leaf_account_index(), Some(LeafAccountIndex::new(0x0f_edcb_a987)?));
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a]?;
    /// assert_eq!(id.leaf_account_index(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn leaf_account_index(self) -> Option<LeafAccountIndex> {
        let depth = usize::from(self.depth());
        if depth & 0x1 == 0 {
            return None;
        }

        let issuance_count = depth / 2;
        let index_offset = mem::size_of::<RawAccountId>() * 8
            - ROOT_ACCOUNT_INDEX_BIT_COUNT
            - (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1) * issuance_count
            - (LEAF_ACCOUNT_INDEX_BIT_COUNT + 1);
        let raw_index = (self.0 >> index_offset) as RawAccountIndex & LEAF_ACCOUNT_INDEX_MASK;

        Some(LeafAccountIndex(raw_index))
    }

    /// Returns an iterator over all of the account index values in this ID, starting with the root
    /// account index and ending with the leaf account index (if any).
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::{
    ///     account_id,
    ///     account::{IssuanceAccountIndex, LeafAccountIndex, RootAccountIndex},
    /// };
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// let mut indexes = id.account_indexes();
    /// assert_eq!(indexes.next(), Some(RootAccountIndex::new(0x42)?.into()));
    /// assert_eq!(indexes.next(), Some(IssuanceAccountIndex::new(0x01_2345)?.into()));
    /// assert_eq!(indexes.next(), Some(IssuanceAccountIndex::new(0x06_789a)?.into()));
    /// assert_eq!(indexes.next(), Some(LeafAccountIndex::new(0x0f_edcb_a987)?.into()));
    /// assert_eq!(indexes.next(), None);
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a]?;
    /// let mut indexes = id.account_indexes();
    /// assert_eq!(indexes.next(), Some(RootAccountIndex::new(0x42)?.into()));
    /// assert_eq!(indexes.next(), Some(IssuanceAccountIndex::new(0x01_2345)?.into()));
    /// assert_eq!(indexes.next(), Some(IssuanceAccountIndex::new(0x06_789a)?.into()));
    /// assert_eq!(indexes.next(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn account_indexes(self) -> iter::AccountIndexIter {
        iter::AccountIndexIter::new(self)
    }

    /// Returns the parent issuance account ID of this account ID, or `None` if this is a root
    /// account ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account_id;
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// assert_eq!(id.parent_id(), Some(account_id![0x42, 0x01_2345, 0x06_789a]?));
    ///
    /// let id = account_id![0x42]?;
    /// assert_eq!(id.parent_id(), None);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn parent_id(self) -> Option<Self> {
        let depth = usize::from(self.depth());
        if depth == 0 {
            return None;
        }

        // If this is an issuance account ID, we will drop down two depth levels instead of one to
        // get to the parent issuance account ID.
        let depth = (depth - 1) & !0x1;

        let issuance_count = depth / 2;
        let chain_bit_count =
            ROOT_ACCOUNT_INDEX_BIT_COUNT + (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1) * issuance_count;
        let chain_shift = mem::size_of::<RawAccountId>() * 8 - chain_bit_count;
        let chain_mask: RawAccountId = ((1 << chain_bit_count) - 1) << chain_shift;

        Some(Self((self.0 & chain_mask) | depth as RawAccountId))
    }

    /// Returns the account ID formed by appending a indicated non-root account index to a fully
    /// formed issuance account ID.
    ///
    /// # Errors
    ///
    /// Returns one of the following [`AccountIdError`] variants if account ID creation fails:
    ///
    /// - [`InvalidDepth`] if the new ID depth would exceed [`MAX_DEPTH`].
    /// - [`NotIssuance`] if `self` is not an issuance account.
    ///
    /// # Examples
    ///
    /// Valid child ID:
    /// ```
    /// use m10_sdk::{account_id, account::{IssuanceAccountIndex, LeafAccountIndex}};
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a]?;
    ///
    /// let child_id = id.child_id(LeafAccountIndex::new(0x0f_edcb_a987)?)?;
    /// assert_eq!(child_id, account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?);
    ///
    /// let child_id = id.child_id(IssuanceAccountIndex::new(0x0f_edcb)?)?;
    /// assert_eq!(child_id, account_id![0x42, 0x01_2345, 0x06_789a, 0x0f_edcb]?);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Failure due to parent not being an issuance account:
    /// ```
    /// use m10_sdk::{account_id, account::LeafAccountIndex};
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// let child_id_res = id.child_id(LeafAccountIndex::new(0x06_5432_1fed)?);
    /// assert!(child_id_res.is_err());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Failure due to account depth limit reached:
    /// ```
    /// use m10_sdk::{account_id, account::IssuanceAccountIndex};
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a, 0x0b_cdef]?;
    /// let child_id_res = id.child_id(IssuanceAccountIndex::new(0x0f_edcb)?);
    /// assert!(child_id_res.is_err());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`InvalidDepth`]: ../error/enum.AccountIdError.html#variant.InvalidDepth
    /// [`MAX_DEPTH`]: #associatedconstant.MAX_DEPTH
    /// [`NotIssuance`]: ../error/enum.AccountIdError.html#variant.NotIssuance
    #[inline]
    pub fn child_id(
        self,
        child_index: impl Into<NonRootAccountIndex>,
    ) -> Result<Self, AccountIdError> {
        let mut builder = Builder::from_id(self)?;
        Ok(match child_index.into() {
            NonRootAccountIndex::Issuance(index) => builder.push(index)?.issuance_id(),
            NonRootAccountIndex::Leaf(index) => builder.leaf_id(index)?,
        })
    }

    /// Returns whether `self` is strictly a descendant of `id` (and not the same ID).
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account_id;
    ///
    /// let parent_id = account_id![0x42, 0x01_2345, 0x06_789a]?;
    /// let child_id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// let unrelated_id = account_id![0x42, 0x01_2345; 0x0f_edcb_a987]?;
    /// assert!(child_id.is_descendant_of(parent_id));
    /// assert!(!child_id.is_descendant_of(unrelated_id));
    /// assert!(!child_id.is_descendant_of(child_id));
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_descendant_of(self, id: AccountId) -> bool {
        if !id.is_issuance() || self.depth() <= id.depth() {
            return false;
        }

        let id_shift = mem::size_of::<RawAccountId>() * 8
            - ROOT_ACCOUNT_INDEX_BIT_COUNT
            - (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1) * (id.depth() as usize / 2);
        let id_mask = u128::MAX << id_shift;
        (self.0 ^ id.0) & id_mask == 0
    }

    /// Returns whether `self` is a descendant of `id` or the same ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account_id;
    ///
    /// let parent_id = account_id![0x42, 0x01_2345, 0x06_789a]?;
    /// let child_id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// let unrelated_id = account_id![0x42, 0x01_2345; 0x0f_edcb_a987]?;
    /// assert!(child_id.is_eq_or_descendant_of(parent_id));
    /// assert!(!child_id.is_eq_or_descendant_of(unrelated_id));
    /// assert!(parent_id.is_eq_or_descendant_of(parent_id));
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_eq_or_descendant_of(self, id: AccountId) -> bool {
        self == id || self.is_descendant_of(id)
    }

    /// Converts this ID to a big-endian byte array.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account_id;
    ///
    /// let id = account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?;
    /// assert_eq!(
    ///     id.to_be_bytes(),
    ///     [
    ///         0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x0f,
    ///         0xed, 0xcb, 0xa9, 0x87, 0x00, 0x00, 0x00, 0x05,
    ///     ]
    /// );
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn to_be_bytes(self) -> [u8; mem::size_of::<RawAccountId>()] {
        self.0.to_be_bytes()
    }

    /// Attempts to create an ID from a big-endian byte array.
    ///
    /// This performs the inverse of [`to_be_bytes`].
    ///
    /// # Errors
    ///
    /// Returns one of the following [`AccountIdError`] variants if account ID creation fails:
    ///
    /// - [`InvalidDepth`] if the account ID depth exceeds [`MAX_DEPTH`].
    /// - [`InvalidSpecialBits`] if any non-index bits (i.e. bits reserved for identifying issuance
    ///   account indexes and unused bits within the ID) are set incorrectly.
    ///
    /// # Examples
    ///
    /// Valid account ID:
    /// ```
    /// use m10_sdk::{account_id, account::AccountId};
    ///
    /// let id = AccountId::try_from_be_bytes([
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x0f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x00, 0x00, 0x00, 0x05,
    /// ])?;
    /// assert_eq!(id, account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Invalid due to depth limit:
    /// ```
    /// use m10_sdk::account::AccountId;
    ///
    /// let id_res = AccountId::try_from_be_bytes([
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x8b,
    ///     0xcd, 0xef, 0x8f, 0xed, 0xcb, 0x00, 0x00, 0x08,
    /// ]);
    /// assert!(id_res.is_err());
    /// ```
    ///
    /// Invalid due to invalid non-index bits:
    /// ```
    /// use m10_sdk::account::AccountId;
    ///
    /// // Leaf account index with issuance bit set.
    /// let id_res = AccountId::try_from_be_bytes([
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x8f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x00, 0x00, 0x00, 0x05,
    /// ]);
    /// assert!(id_res.is_err());
    ///
    /// // Issuance account index with leaf bit set.
    /// let id_res = AccountId::try_from_be_bytes([
    ///     0x42, 0x01, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x0f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x00, 0x00, 0x00, 0x05,
    /// ]);
    /// assert!(id_res.is_err());
    ///
    /// // Garbage data within the ID.
    /// let id_res = AccountId::try_from_be_bytes([
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x0f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x80, 0x00, 0x00, 0x05,
    /// ]);
    /// assert!(id_res.is_err());
    /// ```
    ///
    /// [`to_be_bytes`]: #method.to_be_bytes
    /// [`AccountIdError`]: ../error/enum.AccountIdError.html
    /// [`InvalidDepth`]: ../error/enum.AccountIdError.html#variant.InvalidDepth
    /// [`MAX_DEPTH`]: #associatedconstant.MAX_DEPTH
    /// [`InvalidSpecialBits`]: ../error/enum.AccountIdError.html#variant.InvalidSpecialBits
    #[inline]
    pub fn try_from_be_bytes(
        bytes: [u8; mem::size_of::<RawAccountId>()],
    ) -> Result<Self, AccountIdError> {
        Self::from_raw(RawAccountId::from_be_bytes(bytes))
    }

    /// Attempts to create an ID from a big-endian byte slice.
    ///
    /// This performs the inverse of [`to_be_bytes`] while also verifying the slice length is the
    /// same as the size of [`RawAccountId`].
    ///
    /// # Errors
    ///
    /// Returns one of the following [`AccountIdError`] variants if account ID creation fails:
    ///
    /// - [`InvalidDepth`] if the account ID depth exceeds [`MAX_DEPTH`].
    /// - [`InvalidSpecialBits`] if any non-index bits (i.e. bits reserved for identifying issuance
    ///   account indexes and unused bits within the ID) are set incorrectly.
    /// - [`InvalidLen`] if the slice length does not equal the size of [`RawAccountId`].
    ///
    /// # Examples
    ///
    /// Valid account ID:
    /// ```
    /// use m10_sdk::{account_id, account::AccountId};
    ///
    /// let id = AccountId::try_from_be_slice(&[
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x0f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x00, 0x00, 0x00, 0x05,
    /// ])?;
    /// assert_eq!(id, account_id![0x42, 0x01_2345, 0x06_789a; 0x0f_edcb_a987]?);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Invalid due to depth limit:
    /// ```
    /// use m10_sdk::account::AccountId;
    ///
    /// let id_res = AccountId::try_from_be_slice(&[
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x8b,
    ///     0xcd, 0xef, 0x8f, 0xed, 0xcb, 0x00, 0x00, 0x08,
    /// ]);
    /// assert!(id_res.is_err());
    /// ```
    ///
    /// Invalid due to invalid non-index bits:
    /// ```
    /// use m10_sdk::account::AccountId;
    ///
    /// // Leaf account index with issuance bit set.
    /// let id_res = AccountId::try_from_be_slice(&[
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x8f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x00, 0x00, 0x00, 0x05,
    /// ]);
    /// assert!(id_res.is_err());
    ///
    /// // Issuance account index with leaf bit set.
    /// let id_res = AccountId::try_from_be_slice(&[
    ///     0x42, 0x01, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x0f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x00, 0x00, 0x00, 0x05,
    /// ]);
    /// assert!(id_res.is_err());
    ///
    /// // Garbage data within the ID.
    /// let id_res = AccountId::try_from_be_slice(&[
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x0f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x80, 0x00, 0x00, 0x05,
    /// ]);
    /// assert!(id_res.is_err());
    /// ```
    ///
    /// Invalid due to incorrect slice length:
    /// ```
    /// use m10_sdk::account::AccountId;
    ///
    /// let id_res = AccountId::try_from_be_slice(&[
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x8f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x05,
    /// ]);
    /// assert!(id_res.is_err());
    ///
    /// let id_res = AccountId::try_from_be_slice(&[
    ///     0x42, 0x81, 0x23, 0x45, 0x86, 0x78, 0x9a, 0x8f,
    ///     0xed, 0xcb, 0xa9, 0x87, 0x00, 0x00, 0x00, 0x05,
    ///     0xff,
    /// ]);
    /// assert!(id_res.is_err());
    /// ```
    ///
    /// [`to_be_bytes`]: #method.to_be_bytes
    /// [`RawAccountId`]: ../raw/type.RawAccountId.html
    /// [`AccountIdError`]: ../error/enum.AccountIdError.html
    /// [`InvalidDepth`]: ../error/enum.AccountIdError.html#variant.InvalidDepth
    /// [`MAX_DEPTH`]: #associatedconstant.MAX_DEPTH
    /// [`InvalidSpecialBits`]: ../error/enum.AccountIdError.html#variant.InvalidSpecialBits
    /// [`InvalidLen`]: ../error/enum.AccountIdError.html#variant.InvalidLen
    #[inline]
    pub fn try_from_be_slice(bytes: &[u8]) -> Result<Self, AccountIdError> {
        Self::from_raw(RawAccountId::from_be_bytes(
            bytes.try_into().map_err(|_| AccountIdError::InvalidLen)?,
        ))
    }
}

impl Serialize for AccountId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u128(self.0)
    }
}

impl<'de> Deserialize<'de> for AccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected};

        let raw = RawAccountId::deserialize(deserializer)?;
        let id = AccountId::from_raw(raw).map_err(|error| {
            D::Error::invalid_value(
                Unexpected::Other(&format!("integer `{}`", raw)),
                &format!("a valid account ID ({})", error).as_str(),
            )
        })?;

        Ok(id)
    }
}

impl TryFrom<RawAccountId> for AccountId {
    type Error = AccountIdError;

    #[inline]
    fn try_from(raw: RawAccountId) -> Result<Self, Self::Error> {
        Self::from_raw(raw)
    }
}

impl From<AccountId> for RawAccountId {
    #[inline]
    fn from(id: AccountId) -> Self {
        id.as_raw()
    }
}

impl fmt::Debug for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}", self.root_account_index())?;
        for index in self.issuance_account_indexes() {
            write!(f, ", {}", index)?;
        }

        if let Some(index) = self.leaf_account_index() {
            write!(f, "; {}", index)?;
        }

        write!(f, "]")
    }
}

/// Creates an [`AccountId`] from the given chain of raw account index values.
///
/// Account indexes are specified similar to arrays. The first index is the root account index,
/// followed by zero or more issuance account indexes separated by commas. The last index may be
/// either a leaf account index or issuance account index; to indicate a leaf account, the last
/// index should be separated from the previous indexes using a semicolon instead of a comma. At
/// least one index (the root account index) must be provided, and leaf indexes must always be
/// preceded by at least a root account index.
///
/// Indexes can be either raw values ([`RawAccountIndex`]) or typed values ([`RootAccountIndex`],
/// [`IssuanceAccountIndex`], and [`LeafAccountIndex`]).
///
/// This macro must build the ID from scratch on every invocation. If a parent ID is already known,
/// it is much more efficient to use either the [`child_id`] method or the [incremental ID builder]
/// directly.
///
/// # Errors
///
/// Returns one of the following [`AccountIdError`] variants if account ID creation fails:
///
/// - [`InvalidDepth`] if the account chain length exceeds [`MAX_DEPTH`].
/// - [`IndexRange`] if any of the account indexes exceeds the maximum supported length for that
///   account type.
///
/// # Examples
///
/// Root account index:
/// ```
/// use m10_sdk::account_id;
///
/// let id = account_id![2]?;
/// assert_eq!(id.as_raw(), 0x0200_0000_0000_0000_0000_0000_0000_0000);
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
///
/// Non-root issuance account index:
/// ```
/// use m10_sdk::account_id;
///
/// let id = account_id![2, 34, 56]?;
/// assert_eq!(id.as_raw(), 0x0280_0022_8000_3800_0000_0000_0000_0004);
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
///
/// Leaf account index:
/// ```
/// use m10_sdk::account_id;
///
/// let id = account_id![2, 34, 56; 78]?;
/// assert_eq!(id.as_raw(), 0x0280_0022_8000_3800_0000_004e_0000_0005);
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
///
/// [`AccountId`]: account/id/struct.AccountId.html
/// [`RawAccountIndex`]: account/raw/type.RawAccountIndex.html
/// [`RootAccountIndex`]: account/index/struct.RootAccountIndex.html
/// [`IssuanceAccountIndex`]: account/index/struct.IssuanceAccountIndex.html
/// [`LeafAccountIndex`]: account/index/struct.LeafAccountIndex.html
/// [`child_id`]: account/id/struct.AccountId.html
/// [incremental ID builder]: account/id/struct.Builder.html
/// [`AccountIdError`]: account/error/enum.AccountIdError.html
/// [`InvalidDepth`]: account/error/enum.AccountIdError.html#variant.InvalidDepth
/// [`MAX_DEPTH`]: account/id/struct.AccountId.html#associatedconstant.MAX_DEPTH
/// [`IndexRange`]: account/error/enum.AccountIdError.html#variant.IndexRange
#[macro_export]
macro_rules! account_id {
    // Issuance account ID.
    ($root:expr, $($idx:expr),* $(,)?) => {
        (|| -> ::core::result::Result<_, $crate::account::AccountIdError> {
            #[allow(unused_mut)]
            let mut builder = $crate::account::id::Builder::from_root_account_index($root)?;
            $(builder.push($idx)?;)*
            Ok(builder.issuance_id())
        })()
    };

    ($root:expr) => {
        $crate::account_id!($root,)
    };

    // Leaf account ID.
    ($root:expr, $($idx:expr),*; $leaf:expr $(,)?) => {
        (|| -> ::core::result::Result<_, $crate::account::AccountIdError> {
            #[allow(unused_mut)]
            let mut builder = $crate::account::id::Builder::from_root_account_index($root)?;
            $(builder.push($idx)?;)*
            builder.leaf_id($leaf)
        })()
    };

    ($root:expr; $leaf:expr $(,)?) => {
        $crate::account_id!($root,; $leaf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_id_builder() {
        let mut builder =
            Builder::from_root_account_index(0).expect("Could not establish root account");
        builder
            .push(1)
            .expect("Could not push an issuance account onto the root");
        builder
            .push(2)
            .expect("Could not push an issuance account onto an issuance account");
        builder
            .push(3)
            .expect("Could not push an issuance account onto an issuance account");

        let id_from_builder = builder.issuance_id();

        let id_from_chain = AccountId::from_root_account_index(0)
            .expect("Could not create a root account id")
            .child_id(IssuanceAccountIndex::new(1).expect("Could not create issuance index 1"))
            .expect("Could not get account id from root + issuance 1")
            .child_id(IssuanceAccountIndex::new(2).expect("Could not create issuance index 2"))
            .expect("Could not get account id from root + issuance 1 + issuance 2")
            .child_id(IssuanceAccountIndex::new(3).expect("Could not create issuance index 3"))
            .expect("Could not get account id from root + issuance 1..3");

        assert_eq!(
            id_from_builder, id_from_chain,
            "Got {} vs {}",
            id_from_builder, id_from_chain,
        );

        let id_from_builder = builder
            .leaf_id(10)
            .expect("Could not push a leaf account onto an issuance account");
        let id_from_chain = id_from_chain
            .child_id(LeafAccountIndex::new(10).expect("Could not create leaf index 10"))
            .expect("Could not get account id from root + issuance 1..3 + leaf 10");

        assert_eq!(
            id_from_builder, id_from_chain,
            "Got {} vs {}",
            id_from_builder, id_from_chain,
        );
    }
}
