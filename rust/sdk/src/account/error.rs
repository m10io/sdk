//! Account index and account ID processing errors.

use std::{convert::Infallible, fmt};

/// Error returned by [`TryFrom`] conversions from [`AccountIndex`] to a specific account index type
/// ([`RootAccountIndex`], [`IssuanceAccountIndex`], or [`LeafAccountIndex`]).
///
/// [`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
/// [`AccountIndex`]: ../index/enum.AccountIndex.html
/// [`RootAccountIndex`]: ../index/struct.RootAccountIndex.html
/// [`IssuanceAccountIndex`]: ../index/struct.IssuanceAccountIndex.html
/// [`LeafAccountIndex`]: ../index/struct.LeafAccountIndex.html
#[derive(Debug)]
pub struct TryFromAccountIndexError;

impl fmt::Display for TryFromAccountIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "`AccountIndex` does not contain an index of the target type"
        )
    }
}

impl std::error::Error for TryFromAccountIndexError {}

/// Possible [`AccountId`] construction errors.
///
/// [`AccountId`]: ../id/struct.AccountId.html
#[derive(Debug)]
pub enum AccountIdError {
    /// Missing root account ID/index.
    MissingRoot,

    /// Account chain length exceeds the maximum supported account ID depth.
    InvalidDepth,

    /// Account index exceeds the supported range for the account type.
    IndexRange,

    /// Raw account ID value has invalid non-index bit values (i.e. bits reserved for identifying
    /// issuance account indexes and unused bits within the ID).
    InvalidSpecialBits,

    /// An issuance account ID was expected as the `self` argument for a [`child_id`] call, but a
    /// non-issuance (leaf) account was provided.
    ///
    /// [`child_id`]: ../id/struct.AccountId.html#method.child_id
    NotIssuance,

    /// A leaf account ID was expected as the `self` argument for a [`sibling_id`] call, but a
    /// non-leaf (issuance) account was provided.
    ///
    /// [`sibling_id`]: ../id/struct.AccountId.html#method.sibling_id
    NotLeaf,

    /// The input slice does not have the correct length for [`try_from_be_slice`] conversions.
    ///
    /// [`try_from_be_slice`]: ../id/struct.AccountId.html#method.try_from_be_slice
    InvalidLen,
}

impl fmt::Display for AccountIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingRoot => write!(f, "missing root account ID/index"),
            Self::InvalidDepth => write!(f, "maximum account ID depth exceeded"),
            Self::IndexRange => write!(
                f,
                "account index exceeds supported range for the account type"
            ),
            Self::InvalidSpecialBits => {
                write!(f, "raw account ID value has invalid special bit values")
            }
            Self::NotIssuance => write!(f, "expected issuance account for the base account ID"),
            Self::NotLeaf => write!(f, "expected leaf account for the base account ID"),
            Self::InvalidLen => write!(f, "invalid length for raw ID byte slice"),
        }
    }
}

impl std::error::Error for AccountIdError {}

impl From<Infallible> for AccountIdError {
    #[inline]
    fn from(_error: Infallible) -> Self {
        unreachable!()
    }
}
