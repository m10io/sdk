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

    pub fn updated_at(&mut self, timestamp: u64) -> &mut Self {
        self.document.updated_at = timestamp;
        let path = "updated_at".to_string();
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

    pub fn label(&mut self, key: String, value: String) -> &mut Self {
        self.document.labels.insert(key, value);
        let path = "labels".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn labels(&mut self, labels: std::collections::HashMap<String, String>) -> &mut Self {
        self.document.labels.extend(labels);
        let path = "labels".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn merge_repeated(&mut self, merge_repeated: bool) -> &mut Self {
        self.merge_repeated = merge_repeated;
        self
    }
}
