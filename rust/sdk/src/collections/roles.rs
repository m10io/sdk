use super::*;
use m10_protos::prost::bytes::Bytes;
use m10_protos::sdk::{Role, Rule};

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
