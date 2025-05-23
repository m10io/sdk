use super::*;
use crate::document_id::DocumentId;
use m10_protos::sdk::{Expression, RoleBinding};

impl DocumentUpdate<RoleBinding> {
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

    pub fn subject(&mut self, subject: impl DocumentId) -> &mut Self {
        self.document.subjects.push(subject.into_bytes());
        let path = "subjects".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn subjects<D: DocumentId>(&mut self, subjects: Vec<D>) -> &mut Self {
        self.document
            .subjects
            .extend(&mut subjects.into_iter().map(DocumentId::into_bytes));
        let path = "subjects".to_string();
        if !self.mask.paths.contains(&path) {
            self.mask.paths.push(path);
        }
        self
    }

    pub fn expressions(&mut self, mut expressions: Vec<Expression>) -> &mut Self {
        self.document.expressions.append(&mut expressions);
        let path = "expressions".to_string();
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
