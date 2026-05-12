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

    pub fn status(&mut self, status: i32) -> &mut Self {
        self.document.status = status;
        let path = "status".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn country_code(&mut self, country_code: String) -> &mut Self {
        self.document.country_code = country_code;
        let path = "country_code".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn endpoint(&mut self, endpoint: String) -> &mut Self {
        self.document.endpoint = endpoint;
        let path = "endpoint".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn logo_url(&mut self, logo_url: String) -> &mut Self {
        self.document.logo_url = logo_url;
        let path = "logo_url".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.document.description = description;
        let path = "description".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn bic_swift_code(&mut self, bic_swift_code: String) -> &mut Self {
        self.document.bic_swift_code = bic_swift_code;
        let path = "bic_swift_code".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }
}
