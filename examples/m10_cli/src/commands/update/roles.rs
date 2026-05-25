use clap::Args;
use m10_sdk::{
    prost::bytes::Bytes,
    sdk::{self, rule::Ty},
    DocumentUpdate, WithContext,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};
use uuid::Uuid;

use crate::utils::{parse_key_value, validate_labels, validate_rules};
use crate::{collections::roles::RuleArgs, context::Context};

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateRoleArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Update name field
    #[arg(short, long)]
    name: Option<String>,
    /// Update owner field
    #[arg(short, long)]
    owner: Option<String>,
    /// Set rule
    #[arg(
        short,
        long,
        required = true,
        long_help = concat!("IMPORTANT: When updating one or more rules for a role, ALL rules and ",
        "their verbs must be entered, even those that don't change. Rules include --collections ",
        "(-c), --verbs (-v) and optionally, --instances (-i), expression (--when) and attributes ",
        "(--types). Default collections include ledger-accounts (aka \"account\"), ",
        "account-metadata, roles and role-bindings. Available verbs include Read, ",
        "Create, Update, Delete, Transact, Initiate, and Commit. Instances ",
        "take the argument of account-metadata ID in uuid format. An option key has one argument ",
        "only. E.g.  *-r 'rule -c roles -v Read -v Update -v Delete'*")
    )]
    rule: Vec<RuleArgs>,
    /// Update description field
    #[arg(short = 'd', long)]
    description: Option<String>,
    /// Update labels (e.g. `-l label_1=value_1 -l label_2=value_2`)
    #[arg(
        short = 'l',
        long,
        value_parser = parse_key_value,
        long_help = "IMPORTANT: When updating one or more labels for a role, ALL labels and their values must be entered, even those that don't change.",
    )]
    labels: Option<Vec<(String, String)>>,
}

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateRoleMetadataArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Update name field
    #[arg(short, long)]
    name: Option<String>,
    /// Update owner field
    #[arg(short, long)]
    owner: Option<String>,
    /// Update description field
    #[arg(short = 'd', long)]
    description: Option<String>,
    /// Update labels
    #[arg(short = 'l', long, value_parser = parse_key_value)]
    labels: Option<Vec<(String, String)>>,
}

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateRoleRulesArgs {
    /// Record id or name
    pub(super) id: Uuid,
    /// Set rules via command line
    #[arg(
        short,
        long,
        group = "rules-source",
        long_help = concat!("Rules include --collections (-c), --verbs (-v) and optionally, ",
        "--instances (-i), expression (--when) and attributes (--types). Default collections ",
        "include ledger-accounts (aka \"account\"), account-metadata, roles and role-bindings. ",
        "Available verbs include Read, Create, Update, Delete, Transact, Initiate, Commit, Deny ",
        "and Revoke. Instances take the argument of account-metadata ID in uuid format. An ",
        "option key has one argument only. E.g.  *-r 'rule -c roles -v Read -v Update -v Delete'*")
    )]
    rule: Vec<RuleArgs>,
    /// Provide rule definitions in a file (JSON or YAML)
    #[arg(short, long, group = "rules-source")]
    file: Option<String>,
    /// Edit rules with default editor
    #[arg(short, long, group = "rules-source")]
    editor: bool,
}

impl super::BuildFromArgs for UpdateRoleArgs {
    type Document = sdk::Role;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<bool> {
        let changed = self.owner.is_some()
            || self.name.is_some()
            || !self.rule.is_empty()
            || self.description.is_some()
            || self.labels.is_some();

        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(Bytes::from(owner_key));
        }
        if let Some(name) = self.name {
            builder.name(name);
        }
        if !self.rule.is_empty() {
            validate_rules(self.rule.as_slice())?;
            let rules = self.rule.iter().map(|r| r.to_rbac_rule()).collect();
            builder.rules(rules);
        }
        if let Some(description) = self.description {
            if description.len() > 100 {
                return Err(anyhow::anyhow!(
                    "Description must be 100 characters or less"
                ));
            }
            builder.description(description);
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

impl super::BuildFromArgs for UpdateRoleMetadataArgs {
    type Document = sdk::Role;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<bool> {
        let changed = self.owner.is_some()
            || self.name.is_some()
            || self.description.is_some()
            || self.labels.is_some();

        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(Bytes::from(owner_key));
        }
        if let Some(name) = self.name {
            builder.name(name);
        }
        if let Some(description) = self.description {
            if description.len() > 100 {
                return Err(anyhow::anyhow!(
                    "Description must be 100 characters or less"
                ));
            }
            builder.description(description);
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

impl UpdateRoleRulesArgs {
    pub(crate) async fn update(self, context: &Context) -> anyhow::Result<()> {
        let rules = if !self.rule.is_empty() {
            validate_rules(&self.rule)?;
            self.rule.iter().map(|r| r.to_rbac_rule()).collect()
        } else if let Some(file_path) = &self.file {
            Self::load_rules_from_file(file_path)?
        } else if self.editor {
            Self::edit_rules_interactively(self.id, context).await?
        } else {
            return Err(anyhow::anyhow!(
                "Must provide rules via --rule, --file, or --editor"
            ));
        };

        let mut builder = DocumentUpdate::<sdk::Role>::new(self.id);
        builder.rules(rules);

        builder.merge_repeated(true);

        m10_sdk::documents(
            context.ledger_client(),
            m10_sdk::DocumentBuilder::default()
                .update(&builder)
                .context_id(context.context_id()),
        )
        .await?;

        println!("The rules for role {} have been updated", self.id);

        Ok(())
    }

    fn load_rules_from_file(file_path: &str) -> anyhow::Result<Vec<sdk::Rule>> {
        let path = Path::new(file_path);
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow::anyhow!("File must have .json, .yaml, or .yml extension"))?;

        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let rule_args: Vec<RuleArgs> = match extension {
            "json" => serde_json::from_str(&contents)?,
            "yaml" | "yml" => serde_yml::from_str(&contents)?,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported file extension. Use .json, .yaml, or .yml"
                ))
            }
        };
        validate_rules(&rule_args)?;
        Ok(rule_args.iter().map(|r| r.to_rbac_rule()).collect())
    }

    async fn edit_rules_interactively(
        role_id: Uuid,
        context: &Context,
    ) -> anyhow::Result<Vec<sdk::Rule>> {
        let role = m10_sdk::get_role(context.ledger_client(), role_id).await?;

        let current_rules = Self::convert_rules_to_args(&role.rules)?;

        let mut yaml_content = Self::create_yaml_with_comments(&current_rules)?;

        loop {
            let edited_content = edit::edit(yaml_content.clone())
                .map_err(|e| anyhow::anyhow!("Failed to open editor: {}", e))?;

            match serde_yml::from_str::<Vec<RuleArgs>>(&edited_content) {
                Ok(rule_args) => {
                    if let Err(e) = validate_rules(&rule_args) {
                        eprintln!("Validation error: {}", e);
                        eprintln!("\n[E]dit again or [A]bort? ");

                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input)?;

                        match input.trim().to_lowercase().as_str() {
                            "e" | "edit" => {
                                yaml_content = edited_content;
                                continue;
                            }
                            _ => return Err(anyhow::anyhow!("Aborted by user")),
                        }
                    }

                    return Ok(rule_args.iter().map(|r| r.to_rbac_rule()).collect());
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                    eprintln!("\n[E]dit again or [A]bort? ");

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;

                    match input.trim().to_lowercase().as_str() {
                        "e" | "edit" => {
                            yaml_content = edited_content;
                            continue;
                        }
                        _ => return Err(anyhow::anyhow!("Aborted by user")),
                    }
                }
            }
        }
    }

    fn convert_rules_to_args(rules: &[sdk::Rule]) -> anyhow::Result<Vec<RuleArgs>> {
        rules
            .iter()
            .map(|rule| {
                let instances = if rule.instance_keys.is_empty() {
                    None
                } else {
                    Some(
                        rule.instance_keys
                            .iter()
                            .map(|key| {
                                if let Some(m10_sdk::sdk::value::Value::BytesValue(ref bytes)) = key.value {
                                    Uuid::from_slice(bytes).map_err(|e| {
                                        anyhow::anyhow!(
                                            "Failed to convert instance key to UUID: {}. Use --file mode for non-UUID instance keys.",
                                            e
                                        )
                                    })
                                } else {
                                    Err(anyhow::anyhow!("Instance key is not a bytes value"))
                                }
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    )
                };

                let verbs = rule
                    .verbs
                    .iter()
                    .map(|&v| {
                        crate::collections::roles::Verb::try_from(v)
                            .map_err(|e| anyhow::anyhow!("Invalid verb: {}", e))
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let types = if !rule.types.is_empty() {
                    Some(rule
                        .types
                        .iter()
                        .map(|(name, ty)| {
                            Ok((
                                name.clone(),
                                Ty::try_from(*ty).map_err(|e| anyhow::anyhow!("failed to parse Ty: {e}"))?,
                            ))
                        })
                        .collect::<anyhow::Result<Vec<(String, Ty)>>>()?)
                } else {
                    None
                };

                Ok(RuleArgs {
                    collection: rule.collection.clone(),
                    verbs,
                    when: rule.when.clone(),
                    types,
                    instances,
                })
            })
            .collect()
    }

    fn create_yaml_with_comments(rules: &[RuleArgs]) -> anyhow::Result<String> {
        let mut yaml = r#"# Role Rules Configuration
#
# Each rule defines permissions for a collection.
#
# Fields:
#   collection: The collection name (e.g., 'roles', 'account-metadata')
#   verbs: List of allowed operations (Read, Create, Update, Delete, Transact, Initiate, Commit, Deny, Revoke)
#   instances: (Optional) List of specific UUIDs to restrict access to
#   when: (Optional) Condition that must be met to gain access to resources
#   types: (Optional) Type annotations of custom variables used in when-statement
#
# Example:
# - collection: roles
#   verbs:
#     - Read
#     - Update
#   instances:
#     - <ID>
#   when: "transfer.amount < transfer_limit"
#   types:
#   - [transfer_limit, U64]
#

        "#.to_string();

        let rules_yaml = serde_yml::to_string(rules)?;
        yaml.push_str(&rules_yaml);

        Ok(yaml)
    }
}
