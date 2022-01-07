use super::AccountId;
use m10_sdk_protos::sdk;

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
            .map(|t| {
                match t {
                    sdk::target::Target::AccountId(to_id) => to_id.involves_account(id),
                    // target::Target::AnyAccount(()) => true,
                }
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

impl AccountIdExt for sdk::SetFreezeState {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.account_id.involves_account(id)
    }
}

impl AccountIdExt
    for (
        &sdk::transaction_request_payload::Data,
        &sdk::TransactionResponse,
    )
{
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        use sdk::transaction_request_payload::Data;
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
        }
    }
}

impl AccountIdExt for (&sdk::TransactionRequestPayload, &sdk::TransactionResponse) {
    #[inline]
    fn involves_account(&self, id: AccountId) -> bool {
        self.0
            .data
            .as_ref()
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
