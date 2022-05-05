use clap::Parser;
use serde::{Deserialize, Serialize};

mod actions;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::enum_variant_names)]
#[clap(about)]
pub(super) enum CallSubCommands {
    /// Invokes a registered action
    Action(actions::ActionOptions),
}

impl CallSubCommands {
    pub(super) async fn call(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            CallSubCommands::Action(op) => op.invoke(config).await,
        }
    }
}
