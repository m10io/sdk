use super::*;

use m10_protos::sdk::AccountMetadata;

impl DocumentUpdate<AccountMetadata> {
    fn add_masked_path(&mut self, path: &str) {
        let path = path.to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
    }

    pub fn owner(&mut self, owner: Vec<u8>) -> &mut Self {
        self.document.owner = owner;
        self.add_masked_path("owner");
        self
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.document.name = name;
        self.add_masked_path("name");
        self
    }

    pub fn public_name(&mut self, public_name: String) -> &mut Self {
        self.document.public_name = public_name;
        self.add_masked_path("public_name");
        self
    }

    pub fn profile_image_url(&mut self, profile_image_url: String) -> &mut Self {
        self.document.profile_image_url = profile_image_url;
        self.add_masked_path("profile_image_url");
        self
    }

    pub fn isin(&mut self, isin: String) -> &mut Self {
        self.document.isin = isin;
        self.add_masked_path("isin");
        self
    }

    pub fn dti(&mut self, dti: String) -> &mut Self {
        self.document.dti = dti;
        self.add_masked_path("dti");
        self
    }

    pub fn issuer_bank_id(&mut self, issuer_bank_id: String) -> &mut Self {
        self.document.issuer_bank_id = issuer_bank_id;
        self.add_masked_path("issuer_bank_id");
        self
    }
}
