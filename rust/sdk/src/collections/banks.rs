use super::*;

use m10_protos::sdk::{Bank, BankAccountRef};

impl DocumentUpdate<Bank> {
    pub fn owner(&mut self, owner: Vec<u8>) -> &mut Self {
        self.document.owner = owner;
        let path = "owner".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn short_name(&mut self, name: String) -> &mut Self {
        self.document.short_name = name;
        let path = "short_name".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn display_name(&mut self, display_name: String) -> &mut Self {
        self.document.display_name = display_name;
        let path = "display_name".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn accounts(&mut self, accounts: Vec<BankAccountRef>) -> &mut Self {
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
