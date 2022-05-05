use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct UuidOptions {
    /// Set amount of uuids to be generated
    #[clap(short = 'm', long)]
    multiple: Option<usize>,
}

impl UuidOptions {
    pub(super) fn create(&self) {
        let mul = self.multiple.unwrap_or(1);
        for _ in 0..mul {
            let uuid = Uuid::new_v4();
            println!("{}", uuid);
        }
    }
}
