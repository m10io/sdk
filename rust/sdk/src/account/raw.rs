//! Types and constants for working with raw account index and ID values.

/// Raw account ID value type.
pub type RawAccountId = u128;

/// Raw account index value type.
pub type RawAccountIndex = u64;

/// Number of bits reserved for root account indexes.
pub const ROOT_ACCOUNT_INDEX_BIT_COUNT: usize = 8;
/// Bit mask representing the valid bits in a root account index.
pub const ROOT_ACCOUNT_INDEX_MASK: RawAccountIndex = 0xff;

/// Number of bits reserved for issuance account indexes.
///
/// A bit following the most-significant bit of the index should be reserved. This bit will be set
/// within an [`AccountId`] to ensure leaf accounts are ordered separately from issuance accounts
/// when ordering by the ID itself (essentially matching depth-first ordering of the account tree)
/// and to allow for easy ancestor ID testing in subscription predicates and such.
///
/// [`AccountId`]: struct.AccountId.html
pub const ISSUANCE_ACCOUNT_INDEX_BIT_COUNT: usize = 23;
/// Bit mask representing the valid bits in an issuance account index.
pub const ISSUANCE_ACCOUNT_INDEX_MASK: RawAccountIndex = 0x7f_ffff;

/// Number of bits reserved for leaf account indexes.
///
/// A bit following the most-significant bit of the index should be reserved. This bit will be
/// cleared within an [`AccountId`] to ensure leaf accounts are ordered separately from issuance
/// accounts when ordering by the ID itself (essentially matching depth-first ordering of the
/// account tree) and to allow for easy ancestor ID testing in subscription predicates and such.
///
/// [`AccountId`]: struct.AccountId.html
pub const LEAF_ACCOUNT_INDEX_BIT_COUNT: usize = 39;
/// Bit mask representing the valid bits in a leaf account index.
pub const LEAF_ACCOUNT_INDEX_MASK: RawAccountIndex = 0x7f_ffff_ffff;

/// Account ID depth value type.
pub type AccountIdDepth = u8;
/// Number of bits reserved for account ID depth values.
pub const ACCOUNT_ID_DEPTH_BIT_COUNT: usize = 8;
/// Bit mask representing the valid bits in an account ID depth value.
pub const ACCOUNT_ID_DEPTH_MASK: AccountIdDepth = 0xff;
