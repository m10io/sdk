use super::super::AccountId;

/// Iterator returned by [`AccountId::hierarchy`]
/// The iterator traverses the account ID hierarchy bottom up.
///
/// [`AccountId::hierarchy`]: ../struct.AccountId.html#method.hierarchy
#[derive(Clone, Copy, Debug)]
pub struct AccountHierarchyIter(Option<AccountId>);

impl Iterator for AccountHierarchyIter {
    type Item = AccountId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0;
        self.0 = self.0.and_then(AccountId::parent_id);
        current
    }
}

impl AccountHierarchyIter {
    #[inline]
    pub fn new(id: AccountId) -> Self {
        Self(Some(id))
    }
}
