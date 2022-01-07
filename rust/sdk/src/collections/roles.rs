use super::*;
use m10_sdk_protos::arcadius2::{Role, Rule};
use m10_sdk_protos::prost::bytes::Bytes;

impl DocumentUpdate<Role> {
    pub fn owner(&mut self, owner: Bytes) -> &mut Self {
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

    pub fn rule(&mut self, rule: Rule) -> &mut Self {
        self.document.rules.push(rule);
        let path = "rules".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn rules(&mut self, mut rules: Vec<Rule>) -> &mut Self {
        self.document.rules.append(&mut rules);
        let path = "rules".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }
}
