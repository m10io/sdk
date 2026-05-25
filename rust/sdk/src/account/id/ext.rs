use super::AccountId;
use m10_protos::sdk;

/// Extension trait for [`AccountId`]
pub trait AccountIdExt {
    fn involves_account(&self, id: AccountId) -> bool;
}

impl AccountIdExt for [u8] {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        AccountId::try_from_be_slice(self)
            .map(|x| x.is_eq_or_descendant_of(id))
            .unwrap_or(false)
    }
}

impl AccountIdExt for sdk::FinalizedTransfer {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.transfer_steps.iter().any(|step| {
            step.from_account_id.involves_account(id) || step.to_account_id.involves_account(id)
        })
    }
}

impl AccountIdExt for sdk::CreateTransfer {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.transfer_steps
            .iter()
            .any(|step| step.involves_account(id))
    }
}

impl AccountIdExt for sdk::TransferStep {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.from_account_id.involves_account(id) || self.to_account_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::Target {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.target
            .as_ref()
            .map(|t| match t {
                sdk::target::Target::AccountId(to_id) => to_id.involves_account(id),
                sdk::target::Target::AnyAccount(()) => true,
            })
            .unwrap_or(false)
    }
}

impl AccountIdExt for sdk::Action {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.from_account.involves_account(id)
            || self
                .target
                .as_ref()
                .map(|x| x.involves_account(id))
                .unwrap_or(false)
    }
}

impl AccountIdExt for sdk::InvokeAction {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.from_account.involves_account(id)
            || self
                .target
                .as_ref()
                .map(|x| x.involves_account(id))
                .unwrap_or(false)
    }
}

impl AccountIdExt for sdk::CreateLedgerAccount {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.parent_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::CreateToken {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.account_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::RedeemToken {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.account_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::CreateLock {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.account_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::ReleaseLock {
    #[inline]
    fn involves_account(&self, _id: AccountId) -> bool {
        false
    }
}

impl AccountIdExt for sdk::RedemptionStep {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.holder_account_id.involves_account(id) || self.issuance_account_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::RedeemLocksForCycle {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.steps.iter().any(|step| step.involves_account(id))
    }
}

impl AccountIdExt for sdk::SetFreezeState {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.account_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::SetInstrument {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.account_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::SetDisplayCode {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.account_id.involves_account(id)
    }
}

impl AccountIdExt for sdk::SetBalanceLimit {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.account_id.involves_account(id)
    }
}

impl AccountIdExt for (&sdk::transaction_data::Data, &sdk::TransactionResponse) {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        use sdk::transaction_data::Data;
        match self.0 {
            Data::InvokeAction(action) => action.involves_account(id),
            Data::DocumentOperations(_) => false,
            Data::CreateLedgerAccount(create_account) => create_account.involves_account(id),
            Data::SetFreezeState(set_frozen) => set_frozen.involves_account(id),
            Data::Transfer(transfer) | Data::InitiateTransfer(transfer) => {
                transfer.involves_account(id)
            }
            Data::CommitTransfer(_) => self
                .1
                .transfer_committed
                .as_ref()
                .map(|transfer| transfer.involves_account(id))
                .unwrap_or(false),
            Data::SetInstrument(set_instrument) => set_instrument.involves_account(id),
            Data::SetBalanceLimit(set_balance_limit) => set_balance_limit.involves_account(id),
            Data::SetIssuanceLimit(set_issuance_limit) => {
                set_issuance_limit.account_id.involves_account(id)
            }
            Data::SetDisplayCode(set_display_code) => set_display_code.involves_account(id),
            Data::SetMinCommits(set_min_commits) => set_min_commits.account_id.involves_account(id),
            Data::CreateToken(create_token) => create_token.involves_account(id),
            Data::RedeemToken(redeem_token) => redeem_token.involves_account(id),
            Data::CreateLock(create_lock) => create_lock.involves_account(id),
            Data::ReleaseLock(release_lock) => release_lock.involves_account(id),
            Data::RedeemLocksForCycle(redeem) => redeem.involves_account(id),
        }
    }
}

impl AccountIdExt for (&sdk::TransactionRequestPayload, &sdk::TransactionResponse) {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.0
            .data
            .as_ref()
            .and_then(|x| x.data.as_ref())
            .map(|x| (x, self.1).involves_account(id))
            .unwrap_or(false)
    }
}

impl AccountIdExt for sdk::FinalizedTransaction {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        if let (Some(req), Some(resp)) = (&self.request, &self.response) {
            (req, resp).involves_account(id)
        } else {
            false
        }
    }
}
