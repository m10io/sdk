//! Incremental account ID building support.

use super::AccountId;
use crate::account::*;
use std::{convert::TryInto, mem};

/// Incremental account ID builder.
#[derive(Debug)]
pub struct Builder {
    /// Current raw account ID value without depth stored.
    raw: RawAccountId,

    /// Current account ID depth.
    depth: AccountIdDepth,
}

impl Builder {
    /// Creates a new account ID builder to construct descendant account IDs from the given ID.
    ///
    /// # Errors
    ///
    /// Returns one of the following [`AccountIdError`] variants if builder creation fails:
    ///
    /// - [`NotIssuance`] if `id` is not an issuance account.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::{id, AccountId};
    ///
    /// let id = AccountId::from_raw(0x0181_2345_0000_0000_0000_0000_0000_0002)?;
    /// let builder = id::Builder::from_id(id)?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`AccountIdError`]: ../error/enum.AccountIdError.html
    /// [`NotIssuance`]: ../error/enum.AccountIdError.html#variant.NotIssuance
    pub fn from_id(id: AccountId) -> Result<Self, AccountIdError> {
        if !id.is_issuance() {
            Err(AccountIdError::NotIssuance)
        } else {
            Ok(Self {
                raw: id.as_raw() - (id.as_raw() & ACCOUNT_ID_DEPTH_MASK as RawAccountId),
                depth: id.depth(),
            })
        }
    }

    /// Creates a new account ID builder, starting with the given root account index.
    ///
    /// # Errors
    ///
    /// Returns one of the following [`AccountIdError`] variants if builder creation fails:
    ///
    /// - [`IndexRange`] if `root_index` exceeds the maximum range supported for root account
    ///   indexes.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::id;
    ///
    /// let builder = id::Builder::from_root_account_index(3)?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`AccountIdError`]: ../error/enum.AccountIdError.html
    /// [`IndexRange`]: ../error/enum.AccountIdError.html#variant.IndexRange
    pub fn from_root_account_index<RootIndexT>(
        root_index: RootIndexT,
    ) -> Result<Self, AccountIdError>
    where
        RootIndexT: TryInto<RootAccountIndex>,
        AccountIdError: From<RootIndexT::Error>,
    {
        let root_index = root_index.try_into()?.as_raw();
        Ok(Self {
            raw: RawAccountId::from(root_index)
                << (mem::size_of::<RawAccountId>() * 8 - ROOT_ACCOUNT_INDEX_BIT_COUNT),
            depth: 0,
        })
    }

    /// Pushes a non-root issuance account onto the ID being built.
    ///
    /// # Errors
    ///
    /// Returns one of the following [`AccountIdError`] variants on failure:
    ///
    /// - [`InvalidDepth`] if the new account depth would exceed [`AccountId::MAX_DEPTH`].
    /// - [`IndexRange`] if `issuance_index` exceeds the maximum range supported for non-root
    ///   issuance account indexes.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::id;
    ///
    /// let mut builder = id::Builder::from_root_account_index(3)?;
    /// builder.push(0x01_2345)?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`AccountIdError`]: ../error/enum.AccountIdError.html
    /// [`InvalidDepth`]: ../error/enum.AccountIdError.html#variant.InvalidDepth
    /// [`AccountId::MAX_DEPTH`]: struct.AccountId.html#associatedconstant.MAX_DEPTH
    /// [`IndexRange`]: ../error/enum.AccountIdError.html#variant.IndexRange
    pub fn push<IssuanceIndexT>(
        &mut self,
        issuance_index: IssuanceIndexT,
    ) -> Result<&mut Self, AccountIdError>
    where
        IssuanceIndexT: TryInto<IssuanceAccountIndex>,
        AccountIdError: From<IssuanceIndexT::Error>,
    {
        let issuance_index = issuance_index.try_into()?.as_raw();
        let depth = self.depth + 2;
        if depth > AccountId::MAX_DEPTH {
            Err(AccountIdError::InvalidDepth)
        } else {
            self.depth = depth;
            self.raw |=
                RawAccountId::from(issuance_index | (1 << ISSUANCE_ACCOUNT_INDEX_BIT_COUNT))
                    << (mem::size_of::<RawAccountId>() * 8
                        - ROOT_ACCOUNT_INDEX_BIT_COUNT
                        - (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1) * usize::from(depth / 2));
            Ok(self)
        }
    }

    /// Returns the issuance account ID from the current builder state.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::id;
    ///
    /// let id = id::Builder::from_root_account_index(3)?.push(0x01_2345)?.issuance_id();
    /// assert_eq!(id.as_raw(), 0x0381_2345_0000_0000_0000_0000_0000_0002);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn issuance_id(&self) -> AccountId {
        AccountId(self.raw | RawAccountId::from(self.depth))
    }

    /// Appends a leaf account index to the current ID and returns the resulting leaf account ID.
    ///
    /// # Errors
    ///
    /// Returns one of the following [`AccountIdError`] variants on failure:
    ///
    /// - [`IndexRange`] if `leaf_index` exceeds the maximum range supported for non-root issuance
    ///   account indexes.
    ///
    /// # Examples
    ///
    /// ```
    /// use m10_sdk::account::id;
    ///
    /// let id = id::Builder::from_root_account_index(3)?.push(0x01_2345)?.leaf_id(0x0f_edcb_a987)?;
    /// assert_eq!(id.as_raw(), 0x0381_2345_0fed_cba9_8700_0000_0000_0003);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`AccountIdError`]: ../error/enum.AccountIdError.html
    /// [`IndexRange`]: ../error/enum.AccountIdError.html#variant.IndexRange
    pub fn leaf_id<LeafIndexT>(&self, leaf_index: LeafIndexT) -> Result<AccountId, AccountIdError>
    where
        LeafIndexT: TryInto<LeafAccountIndex>,
        AccountIdError: From<LeafIndexT::Error>,
    {
        let leaf_index = leaf_index.try_into()?.as_raw();
        let depth = self.depth + 1;

        // We should never have a maximum depth that allows issuance accounts without leaf accounts.
        debug_assert!(depth <= AccountId::MAX_DEPTH);

        let raw = self.raw
            | RawAccountId::from(leaf_index)
                << (mem::size_of::<RawAccountId>() * 8
                    - ROOT_ACCOUNT_INDEX_BIT_COUNT
                    - (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1) * usize::from(depth / 2)
                    - (LEAF_ACCOUNT_INDEX_BIT_COUNT + 1));

        Ok(AccountId(raw | RawAccountId::from(depth)))
    }
}
