//! `IssuanceAccountIndexIter` type.

use super::super::*;

/// Iterator returned by [`AccountId::issuance_account_indexes`] over the non-root issuance account
/// indexes in an [`AccountId`].
///
/// [`AccountId::issuance_account_indexes`]: ../struct.AccountId.html#method.issuance_account_indexes
/// [`AccountId`]: ../struct.AccountId.html
#[derive(Clone, Copy, Debug)]
pub struct IssuanceAccountIndexIter {
    /// Account ID value.
    id: AccountId,

    /// "Front" iterator bit shift. This value is pre-decremented when advanced.
    front_shift: usize,

    /// "Back" iterator bit shift. This value is post-incremented when advanced.
    back_shift: usize,
}

impl IssuanceAccountIndexIter {
    /// Creates a new `IssuanceAccountIndexIter` over the non-root issuance account index values in
    /// the given account ID.
    #[inline]
    pub(in super::super) fn new(id: AccountId) -> Self {
        let issuance_count = usize::from(id.depth()) / 2;
        let front_shift = mem::size_of::<RawAccountId>() * 8 - ROOT_ACCOUNT_INDEX_BIT_COUNT;
        let back_shift = front_shift - (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1) * issuance_count;

        Self {
            id,
            front_shift,
            back_shift,
        }
    }

    /// Returns the [`AccountId`] containing the issuance account indexes over which this is
    /// iterating.
    ///
    /// [`AccountId`]: ../struct.AccountId.html
    #[inline]
    pub fn id(self) -> AccountId {
        self.id
    }

    /// Returns the issuance account index at the specified bit offset within the account ID.
    #[inline]
    fn index_at_offset(&self, shift: usize) -> IssuanceAccountIndex {
        IssuanceAccountIndex(
            (self.id.as_raw() >> shift) as RawAccountIndex & ISSUANCE_ACCOUNT_INDEX_MASK,
        )
    }
}

impl Iterator for IssuanceAccountIndexIter {
    type Item = IssuanceAccountIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_shift > self.back_shift {
            self.front_shift -= ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1;
            Some(self.index_at_offset(self.front_shift))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.front_shift - self.back_shift) / (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1);
        (size, Some(size))
    }
}

impl ExactSizeIterator for IssuanceAccountIndexIter {}

impl DoubleEndedIterator for IssuanceAccountIndexIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front_shift > self.back_shift {
            let index = self.index_at_offset(self.back_shift);
            self.back_shift += ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1;
            Some(index)
        } else {
            None
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let n = n * (ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1);
        if n < self.front_shift - self.back_shift {
            let shift = self.back_shift + n;
            let index = self.index_at_offset(shift);
            self.back_shift = shift + ISSUANCE_ACCOUNT_INDEX_BIT_COUNT + 1;
            Some(index)
        } else {
            self.back_shift = self.front_shift;
            None
        }
    }
}

impl FusedIterator for IssuanceAccountIndexIter {}
