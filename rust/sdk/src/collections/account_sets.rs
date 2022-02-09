use super::*;
use m10_protos::sdk::{AccountRef, AccountSet};

impl DocumentUpdate<AccountSet> {
    pub fn owner(&mut self, owner: Vec<u8>) -> &mut Self {
        self.document.owner = owner;
        let path = "owner".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn account(&mut self, account: AccountRef) -> &mut Self {
        self.document.accounts.push(account);
        let path = "accounts".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn accounts(&mut self, accounts: Vec<AccountRef>) -> &mut Self {
        for account in accounts {
            self.document.accounts.push(account);
        }
        let path = "accounts".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }
}
