//! `AccountIndexIter` type.

use super::{super::*, IssuanceAccountIndexIter};
use std::mem;

/// [`AccountIndexIter`] state.
///
/// [`AccountIndexIter`]: struct.AccountIndexIter.html
enum State {
    /// Next value is the root account index.
    Root(AccountId),

    /// Next value is an issuance account index (or leaf index if this iterator is done).
    Issuance(IssuanceAccountIndexIter),

    /// Next value is the leaf account index.
    Leaf(AccountId),

    /// Iterator has completed.
    Done,
}

/// Iterator returned by [`AccountId::account_indexes`] over all of the account index values in an
/// [`AccountId`], starting with the root account index and ending with the leaf account index (if
/// any).
///
/// [`AccountId::account_indexes`]: ../struct.AccountId.html#method.account_indexes
/// [`AccountId`]: ../struct.AccountId.html
pub struct AccountIndexIter {
    /// Iterator state.
    state: State,
}

impl AccountIndexIter {
    /// Creates a new `AccountIndexIter` over the account index values in the given account ID.
    #[inline]
    pub(in super::super) fn new(id: AccountId) -> Self {
        Self {
            state: State::Root(id),
        }
    }
}

impl Iterator for AccountIndexIter {
    type Item = AccountIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match mem::replace(&mut self.state, State::Done) {
            State::Root(id) => {
                let index = id.root_account_index();
                self.state = State::Issuance(id.issuance_account_indexes());
                Some(index.into())
            }

            State::Issuance(mut iter) => match iter.next() {
                Some(index) => {
                    self.state = State::Issuance(iter);
                    Some(index.into())
                }

                None => {
                    self.state = State::Leaf(iter.id());
                    self.next()
                }
            },

            State::Leaf(id) => {
                let index_opt = id.leaf_account_index();
                index_opt.map(|index| index.into())
            }

            State::Done => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = match &self.state {
            State::Root(id) => {
                1 + id.issuance_account_indexes().len()
                    + id.leaf_account_index().map(|_| 1).unwrap_or(0)
            }
            State::Issuance(iter) => {
                iter.len() + iter.id().leaf_account_index().map(|_| 1).unwrap_or(0)
            }
            State::Leaf(id) => id.leaf_account_index().map(|_| 1).unwrap_or(0),
            State::Done => 0,
        };

        (size, Some(size))
    }
}

impl ExactSizeIterator for AccountIndexIter {}

impl FusedIterator for AccountIndexIter {}
