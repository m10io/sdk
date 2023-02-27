use super::*;

use m10_protos::sdk::AccountMetadata;

impl DocumentUpdate<AccountMetadata> {
    pub fn owner(&mut self, owner: Vec<u8>) -> &mut Self {
        self.document.owner = owner;
        let path = "owner".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.document.name = name;
        let path = "name".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn public_name(&mut self, public_name: String) -> &mut Self {
        self.document.public_name = public_name;
        let path = "public_name".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn profile_image_url(&mut self, profile_image_url: String) -> &mut Self {
        self.document.profile_image_url = profile_image_url;
        let path = "profile_image_url".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }
}
