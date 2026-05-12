use crate::collections::roles::RuleArgs;
use crate::utils::{parse_labels, validate_labels, validate_rules};
use clap::Args;
use m10_sdk::{sdk, PublicKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RoleTemplate {
    name: Option<String>,
    description: Option<String>,
    owner: Option<String>,
    rules: Vec<RuleArgs>,
    immutable: Option<bool>,
    labels: std::collections::HashMap<String, String>,
}

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct CreateRoleArgs {
    /// Ignore error if item exists
    #[arg(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set record uuid
    #[arg(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set name of role
    #[arg(short, long, default_value_t)]
    #[serde(default)]
    name: String,
    /// Set owner of the role record
    #[arg(short, long)]
    owner: Option<PublicKey>,
    /// Set rule.
    #[arg(
        short,
        long,
        required_unless_present = "editor",
        long_help = "Rules include --collections (-c), --verbs (-v) and optionally, --instances (-i). Default collections include ledger-accounts (aka \"account\"), account-metadata, roles and role-bindings. Available verbs include Read, Create, Update, Delete, Transact, Initiate, Commit, Grant, Deny, and Revoke. Instances take the argument of account-metadata ID in uuid format. An option key has one argument only. E.g.  *-r 'rule -c roles -v Read -v Update -v Delete'*"
    )]
    rule: Vec<RuleArgs>,
    #[arg(short = 'd', long)]
    description: Option<String>,
    #[arg(long)]
    #[serde(default)]
    editor: bool,
    #[arg(long)]
    #[serde(default)]
    immutable: bool,
    /// Set optional labels. (e.g. `-l label_1=value_1 -l label_2=value_2`)
    #[arg(short = 'l', long, value_parser = parse_labels)]
    #[serde(default)]
    pub labels: Option<Vec<(String, String)>>,
}

impl CreateRoleArgs {
    fn create_yaml_template() -> String {
        let mut yaml = String::new();
        yaml.push_str("# Role Configuration\n");
        yaml.push_str("# \n");
        yaml.push_str("# Metadata Fields (all optional if provided via CLI):\n");
        yaml.push_str("#   name: Role name\n");
        yaml.push_str("#   description: Role description (max 100 characters)\n");
        yaml.push_str("#   owner: Owner's public key in base64 format\n");
        yaml.push_str("# \n");
        yaml.push_str("# Rules (required):\n");
        yaml.push_str("#   Each rule defines permissions for a collection.\n");
        yaml.push_str("#   Fields:\n");
        yaml.push_str("#     collection: Collection name (e.g., 'roles', 'account-metadata', 'ledger-accounts')\n");
        yaml.push_str("#     verbs: List of operations (Read, Create, Update, Delete, Transact, Initiate, Commit, Deny, Revoke)\n");
        yaml.push_str("#       Note: Transact, Initiate, and Commit verbs only apply to 'ledger-accounts' (aka 'account')\n");
        yaml.push_str("#     instances: (Optional) List of specific UUIDs to restrict access to\n");
        yaml.push_str("# \n");
        yaml.push_str("# Example:\n");
        yaml.push_str("# name: example-role\n");
        yaml.push_str("# description: A role for managing accounts\n");
        yaml.push_str("# rules:\n");
        yaml.push_str("#   - collection: account-metadata\n");
        yaml.push_str("#     verbs:\n");
        yaml.push_str("#       - Read\n");
        yaml.push_str("#       - Update\n");
        yaml.push_str("#     instances:\n");
        yaml.push_str("#       - <ID>\n");
        yaml.push_str("#   - collection: ledger-accounts\n");
        yaml.push_str("#     verbs:\n");
        yaml.push_str("#       - Read\n");
        yaml.push_str("#       - Transact\n");
        yaml.push_str("#     instances:\n");
        yaml.push_str("#       - <ID>\n");
        yaml.push_str("# \n");
        yaml.push('\n');
        yaml.push_str("# Uncomment and fill in metadata fields if not provided via CLI:\n");
        yaml.push_str("# name: \"\"\n");
        yaml.push_str("# description: \"\"\n");
        yaml.push_str("# owner: \"\"\n");
        yaml.push('\n');
        yaml.push_str("rules:\n");
        yaml.push_str("  - collection: \"\"\n");
        yaml.push_str("    verbs: []\n");
        yaml
    }

    fn edit_role_interactively(&self) -> anyhow::Result<RoleTemplate> {
        let mut yaml_content = Self::create_yaml_template();

        loop {
            let edited_content = edit::edit(yaml_content.clone())
                .map_err(|e| anyhow::anyhow!("Failed to open editor: {}", e))?;

            match serde_yml::from_str::<RoleTemplate>(&edited_content) {
                Ok(template) => {
                    if let Err(e) = validate_rules(&template.rules) {
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

                    return Ok(template);
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

    fn check_metadata_duplication(&self, template: &RoleTemplate) -> anyhow::Result<()> {
        let mut conflicts = Vec::new();

        if template
            .name
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
            && !self.name.is_empty()
        {
            conflicts.push("name");
        }
        if template
            .description
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
            && self.description.is_some()
        {
            conflicts.push("description");
        }
        if template
            .owner
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
            && self.owner.is_some()
        {
            conflicts.push("owner");
        }

        if !conflicts.is_empty() {
            return Err(anyhow::anyhow!(
                "You can't specify role metadata in both the file and on the command line. Conflicting fields: {}",
                conflicts.join(", ")
            ));
        }

        Ok(())
    }
}

impl super::BuildFromArgs for CreateRoleArgs {
    type Document = sdk::Role;
    fn build_from_options(self, default_owner: PublicKey) -> Result<Self::Document, anyhow::Error> {
        let (name, description, owner, rules, immutable, labels): (
            String,
            String,
            PublicKey,
            Vec<sdk::Rule>,
            bool,
            std::collections::HashMap<String, String>,
        ) = if self.editor {
            let template = self.edit_role_interactively()?;
            self.check_metadata_duplication(&template)?;

            let name = match template.name.as_deref() {
                Some(s) if !s.trim().is_empty() => s.to_string(),
                _ => {
                    if self.name.is_empty() {
                        String::new()
                    } else {
                        self.name.clone()
                    }
                }
            };

            let description = match template.description.as_deref() {
                Some(s) if !s.trim().is_empty() => s.to_string(),
                _ => self.description.unwrap_or_default(),
            };

            let owner = match template.owner.as_deref() {
                Some(s) if !s.trim().is_empty() => {
                    let owner_bytes = base64::decode(s)
                        .map_err(|e| anyhow::anyhow!("Invalid owner public key in file: {}", e))?;
                    PublicKey(owner_bytes)
                }
                _ => self.owner.unwrap_or(default_owner.clone()),
            };

            let rules: Vec<sdk::Rule> = template.rules.iter().map(|r| r.to_rbac_rule()).collect();
            let immutable = template.immutable.unwrap_or(self.immutable);
            let labels = template.labels;

            (name, description, owner, rules, immutable, labels)
        } else {
            validate_rules(&self.rule)?;
            let rules = self.rule.iter().map(|r| r.to_rbac_rule()).collect();
            let description = self.description.unwrap_or_default();
            let owner = self.owner.unwrap_or(default_owner.clone());
            let immutable = self.immutable;
            let labels = self.labels.unwrap_or_default().into_iter().collect();
            (self.name, description, owner, rules, immutable, labels)
        };

        if description.len() > 100 {
            return Err(anyhow::anyhow!(
                "Description must be 100 characters or less"
            ));
        }

        validate_labels(&labels)?;

        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let signer_key = default_owner.0;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64;

        Ok(sdk::Role {
            id: id.into(),
            owner: owner.0.into(),
            name,
            rules,
            created_at: timestamp,
            updated_at: timestamp,
            created_by: signer_key.into(),
            description,
            labels,
            immutable,
        })
    }
}
