use clap::{Args, Subcommand};
use m10_sdk::{prost::bytes::Bytes, sdk, DocumentBuilder, DocumentId, DocumentUpdate, WithContext};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use crate::collections::role_bindings::Expression;
use crate::utils::{parse_labels, validate_labels};

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateRoleBindingArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Update name field
    #[arg(short, long)]
    name: Option<String>,
    /// Update owner field
    #[arg(short, long)]
    owner: Option<String>,
    /// Update role link
    #[arg(short, long)]
    role: Option<Uuid>,
    /// Update guard expression
    #[arg(short, long, alias = "exps")]
    expressions: Option<Expression>,
    /// Update description field
    #[arg(short = 'd', long)]
    description: Option<String>,
    /// Update expiry time in RFC3339 format (YYYY-MM-DDTHH:MM:SSZ). Pass 'never' to clear expiry
    #[arg(long)]
    expires_at: Option<String>,
    /// Update labels (e.g. `-l label_1=value_1 -l label_2=value_2`)
    #[arg(
        short = 'l',
        long,
        value_parser = parse_labels,
        long_help = "IMPORTANT: When updating one or more labels for a role, ALL labels and their values must be entered, even those that don't change.",
    )]
    pub labels: Option<Vec<(String, String)>>,
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
pub(crate) enum RoleBindingSubjects {
    /// Add subjects to a role binding
    #[command(alias = "a")]
    Add(RoleBindingAddSubjectsArgs),
    /// Remove subjects from a role binding
    #[command(alias = "r")]
    Remove(RoleBindingRemoveSubjectsArgs),
}

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct RoleBindingAddSubjectsArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Subjects to add
    #[arg(long = "subject", alias = "subjs")]
    subjects: Vec<String>,
}

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct RoleBindingRemoveSubjectsArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Subjects to remove
    #[arg(long = "subject", alias = "subjs")]
    subjects: Vec<String>,
}

impl RoleBindingSubjects {
    pub(crate) async fn update(self, context: &crate::context::Context) -> anyhow::Result<()> {
        let client = context.ledger_client();

        let mut role_binding = client.get_role_binding(self.id()).await?;

        let decoded_subjects = self.subjects()?;
        if matches!(self, RoleBindingSubjects::Add(_)) {
            if decoded_subjects
                .iter()
                .any(|s| role_binding.subjects.contains(s))
            {
                anyhow::bail!("One or more subjects already exist");
            }
            role_binding.subjects.extend(decoded_subjects);
        } else {
            if decoded_subjects
                .iter()
                .any(|s| !role_binding.subjects.contains(s))
            {
                anyhow::bail!("One or more subjects do not exist");
            }
            let to_remove: HashSet<_> = decoded_subjects.into_iter().collect();
            role_binding.subjects.retain(|s| !to_remove.contains(s));
        }

        let mut builder = DocumentUpdate::<sdk::RoleBinding>::new(role_binding.id);
        builder.subjects(role_binding.subjects);

        builder.merge_repeated(true);

        m10_sdk::documents(
            context.ledger_client(),
            DocumentBuilder::default()
                .update(&builder)
                .context_id(context.context_id()),
        )
        .await?;

        println!("Role binding subjects updated successfully");

        Ok(())
    }

    fn id(&self) -> Vec<u8> {
        match self {
            RoleBindingSubjects::Add(args) => args.id.into_vec(),
            RoleBindingSubjects::Remove(args) => args.id.into_vec(),
        }
    }

    fn subjects(&self) -> Result<Vec<Bytes>, base64::DecodeError> {
        match self {
            RoleBindingSubjects::Add(args) => args
                .subjects
                .iter()
                .map(|s| base64::decode(s).map(Bytes::from))
                .collect(),
            RoleBindingSubjects::Remove(args) => args
                .subjects
                .iter()
                .map(|s| base64::decode(s).map(Bytes::from))
                .collect(),
        }
    }
}

impl super::BuildFromArgs for UpdateRoleBindingArgs {
    type Document = sdk::RoleBinding;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<bool> {
        let changed = self.owner.is_some()
            || self.name.is_some()
            || self.role.is_some()
            || self.expressions.is_some()
            || self.description.is_some()
            || self.expires_at.is_some()
            || self.labels.is_some();

        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(Bytes::from(owner_key));
        }
        if let Some(name) = self.name {
            builder.name(name);
        }
        if let Some(role) = self.role {
            builder.role(role.as_bytes().to_vec().into());
        }
        if let Some(expressions) = self.expressions {
            builder.expressions(
                expressions
                    .0
                    .into_iter()
                    .map(|(collection, expression)| sdk::Expression {
                        collection,
                        expression,
                    })
                    .collect(),
            );
        }
        if let Some(description) = self.description {
            if description.len() > 100 {
                return Err(anyhow::anyhow!(
                    "Description must be 100 characters or less"
                ));
            }
            builder.description(description);
        }
        if let Some(s) = self.expires_at {
            let ms = match s.as_str() {
                "never" => None,
                s => Some(
                    chrono::DateTime::parse_from_rfc3339(s)
                        .map_err(|e| anyhow::anyhow!("Invalid expires_at format (expected RFC3339 YYYY-MM-DDTHH:MM:SSZ): {}", e))?
                        .timestamp_millis() as u64,
                ),
            };
            builder.expires_at(ms);
        }
        if let Some(labels) = self.labels {
            let labels: std::collections::HashMap<String, String> = labels.into_iter().collect();
            validate_labels(&labels)?;
            builder.labels(labels);
        }
        builder.merge_repeated(true);
        Ok(changed)
    }
}
