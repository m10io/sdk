use crate::types::PublicKey;
use crate::{DocumentId, OwnerFilter};
use m10_protos::sdk::{self, MapWrapper};

pub struct PageBuilder<ID: DocumentId, T = ()> {
    filter: T,
    limit: Option<u32>,
    last_id: Option<ID>,
}

impl<ID: DocumentId, T: Default> Default for PageBuilder<ID, T> {
    fn default() -> Self {
        Self {
            filter: T::default(),
            limit: None,
            last_id: None,
        }
    }
}

impl<ID: DocumentId, T> PageBuilder<ID, T> {
    pub fn filter(filter: T) -> Self {
        Self {
            filter,
            limit: None,
            last_id: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn last_id(mut self, id: ID) -> Self {
        self.last_id = Some(id);
        self
    }
}

impl<ID: DocumentId> From<PageBuilder<ID, ()>> for Option<sdk::Page> {
    fn from(builder: PageBuilder<ID, ()>) -> Self {
        if builder.limit.is_none() && builder.last_id.is_none() {
            None
        } else {
            Some(sdk::Page {
                limit: builder.limit.unwrap_or(20),
                last_id: builder
                    .last_id
                    .map(DocumentId::into_vec)
                    .unwrap_or_default(),
            })
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct NameFilter {
    name: String,
}

impl<ID: DocumentId> PageBuilder<ID, NameFilter> {
    pub fn name(name: impl Into<String>) -> Self {
        PageBuilder {
            filter: NameFilter { name: name.into() },
            limit: None,
            last_id: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum NameOrInstanceFilter {
    Name(String),
    InstanceId(Vec<u8>),
}

impl<ID: DocumentId> PageBuilder<ID, NameOrInstanceFilter> {
    pub fn name(name: impl Into<String>) -> Self {
        PageBuilder::filter(NameOrInstanceFilter::Name(name.into()))
    }

    pub fn instance_id(instance_id: impl Into<Vec<u8>>) -> Self {
        PageBuilder::filter(NameOrInstanceFilter::InstanceId(instance_id.into()))
    }
}

#[derive(Clone, Debug)]
pub enum NameOrSubjectFilter {
    Name(String),
    Subject(Vec<u8>),
}

impl<ID: DocumentId> PageBuilder<ID, NameOrSubjectFilter> {
    pub fn name(name: impl Into<String>) -> Self {
        PageBuilder::filter(NameOrSubjectFilter::Name(name.into()))
    }

    pub fn subject(subject: impl Into<Vec<u8>>) -> Self {
        PageBuilder::filter(NameOrSubjectFilter::Subject(subject.into()))
    }
}

#[derive(Clone, Debug)]
pub enum RoleFilter {
    Name(String),
    Description(String),
    Labels(std::collections::HashMap<String, String>),
    InstanceId(Vec<u8>),
}

impl<ID: DocumentId> PageBuilder<ID, RoleFilter> {
    pub fn name(name: impl Into<String>) -> Self {
        PageBuilder::filter(RoleFilter::Name(name.into()))
    }

    pub fn description(description: impl Into<String>) -> Self {
        PageBuilder::filter(RoleFilter::Description(description.into()))
    }

    pub fn labels(labels: impl Into<std::collections::HashMap<String, String>>) -> Self {
        PageBuilder::filter(RoleFilter::Labels(labels.into()))
    }

    pub fn instance_id(instance_id: impl Into<Vec<u8>>) -> Self {
        PageBuilder::filter(RoleFilter::InstanceId(instance_id.into()))
    }
}

#[derive(Clone, Debug)]
pub enum RoleBindingFilter {
    Name(String),
    Description(String),
    Subject(Vec<u8>),
    Labels(std::collections::HashMap<String, String>),
}

impl<ID: DocumentId> PageBuilder<ID, RoleBindingFilter> {
    pub fn name(name: impl Into<String>) -> Self {
        PageBuilder::filter(RoleBindingFilter::Name(name.into()))
    }

    pub fn description(description: impl Into<String>) -> Self {
        PageBuilder::filter(RoleBindingFilter::Description(description.into()))
    }

    pub fn subject(subject: impl Into<Vec<u8>>) -> Self {
        PageBuilder::filter(RoleBindingFilter::Subject(subject.into()))
    }

    pub fn labels(subject: impl Into<std::collections::HashMap<String, String>>) -> Self {
        PageBuilder::filter(RoleBindingFilter::Labels(subject.into()))
    }
}

#[derive(Clone, Debug)]
pub enum NameOrOwnerFilter {
    Name(String),
    Owner(PublicKey),
}

impl<ID: DocumentId> PageBuilder<ID, NameOrOwnerFilter> {
    pub fn name(name: impl Into<String>) -> Self {
        PageBuilder::filter(NameOrOwnerFilter::Name(name.into()))
    }

    pub fn owner(owner: PublicKey) -> Self {
        PageBuilder::filter(NameOrOwnerFilter::Owner(owner))
    }
}

#[derive(Clone, Debug)]
pub enum AccountMetadataFilter {
    Name(String),
    Owner(PublicKey),
    PublicName(String),
    Isin(String),
    Dti(String),
    IssuerBankId(String),
}

impl<ID: DocumentId> PageBuilder<ID, AccountMetadataFilter> {
    pub fn name(name: impl Into<String>) -> Self {
        PageBuilder::filter(AccountMetadataFilter::Name(name.into()))
    }

    pub fn owner(owner: PublicKey) -> Self {
        PageBuilder::filter(AccountMetadataFilter::Owner(owner))
    }

    pub fn public_name(public_name: impl Into<String>) -> Self {
        PageBuilder::filter(AccountMetadataFilter::PublicName(public_name.into()))
    }

    pub fn isin(isin: impl Into<String>) -> Self {
        PageBuilder::filter(AccountMetadataFilter::Isin(isin.into()))
    }

    pub fn dti(dti: impl Into<String>) -> Self {
        PageBuilder::filter(AccountMetadataFilter::Dti(dti.into()))
    }

    pub fn issuer_bank_id(issuer_bank_id: impl Into<String>) -> Self {
        PageBuilder::filter(AccountMetadataFilter::IssuerBankId(issuer_bank_id.into()))
    }
}

impl From<NameOrOwnerFilter> for sdk::list_account_sets_request::Filter {
    fn from(filter: NameOrOwnerFilter) -> Self {
        match filter {
            NameOrOwnerFilter::Name(name) => sdk::list_account_sets_request::Filter::Name(name),
            NameOrOwnerFilter::Owner(owner) => {
                sdk::list_account_sets_request::Filter::Owner(owner.0)
            }
        }
    }
}

impl From<AccountMetadataFilter> for sdk::list_account_metadata_request::Filter {
    fn from(filter: AccountMetadataFilter) -> Self {
        match filter {
            AccountMetadataFilter::Name(name) => {
                sdk::list_account_metadata_request::Filter::Name(name)
            }
            AccountMetadataFilter::Owner(owner) => {
                sdk::list_account_metadata_request::Filter::Owner(owner.0)
            }
            AccountMetadataFilter::PublicName(public_name) => {
                sdk::list_account_metadata_request::Filter::PublicName(public_name)
            }
            AccountMetadataFilter::Isin(isin) => {
                sdk::list_account_metadata_request::Filter::Isin(isin)
            }
            AccountMetadataFilter::IssuerBankId(issuer_bank_id) => {
                sdk::list_account_metadata_request::Filter::IssuerBankId(issuer_bank_id)
            }
            AccountMetadataFilter::Dti(dti) => sdk::list_account_metadata_request::Filter::Dti(dti),
        }
    }
}

impl<ID: DocumentId> From<PageBuilder<ID, RoleFilter>> for sdk::ListRolesRequest {
    fn from(builder: PageBuilder<ID, RoleFilter>) -> Self {
        let page = PageBuilder::<ID, ()> {
            filter: (),
            limit: builder.limit,
            last_id: builder.last_id,
        }
        .into();
        Self {
            page,
            filter: Some(match builder.filter {
                RoleFilter::Name(name) => sdk::list_roles_request::Filter::Name(name),
                RoleFilter::Description(description) => {
                    sdk::list_roles_request::Filter::Description(description)
                }
                RoleFilter::Labels(labels) => {
                    sdk::list_roles_request::Filter::Labels(MapWrapper { value: labels })
                }
                RoleFilter::InstanceId(instance_id) => {
                    sdk::list_roles_request::Filter::InstanceId(instance_id)
                }
            }),
        }
    }
}

impl<ID: DocumentId> From<PageBuilder<ID, RoleBindingFilter>> for sdk::ListRoleBindingsRequest {
    fn from(builder: PageBuilder<ID, RoleBindingFilter>) -> Self {
        let page = PageBuilder::<ID, ()> {
            filter: (),
            limit: builder.limit,
            last_id: builder.last_id,
        }
        .into();
        Self {
            page,
            filter: Some(match builder.filter {
                RoleBindingFilter::Name(name) => {
                    sdk::list_role_bindings_request::Filter::Name(name)
                }
                RoleBindingFilter::Description(description) => {
                    sdk::list_role_bindings_request::Filter::Description(description)
                }
                RoleBindingFilter::Labels(labels) => {
                    sdk::list_role_bindings_request::Filter::Labels(MapWrapper { value: labels })
                }
                RoleBindingFilter::Subject(subject) => {
                    sdk::list_role_bindings_request::Filter::Subject(subject)
                }
            }),
        }
    }
}

impl<ID: DocumentId> From<PageBuilder<ID, NameOrOwnerFilter>> for sdk::ListAccountSetsRequest {
    fn from(builder: PageBuilder<ID, NameOrOwnerFilter>) -> Self {
        let page = PageBuilder::<ID, ()> {
            filter: (),
            limit: builder.limit,
            last_id: builder.last_id,
        }
        .into();
        Self {
            page,
            filter: Some(builder.filter.into()),
        }
    }
}

impl<ID: DocumentId> From<PageBuilder<ID, AccountMetadataFilter>>
    for sdk::ListAccountMetadataRequest
{
    fn from(builder: PageBuilder<ID, AccountMetadataFilter>) -> Self {
        let page = PageBuilder::<ID, ()> {
            filter: (),
            limit: builder.limit,
            last_id: builder.last_id,
        }
        .into();
        Self {
            page,
            filter: Some(builder.filter.into()),
        }
    }
}

impl<ID: DocumentId> PageBuilder<ID, OwnerFilter> {
    pub fn owner(owner: Vec<u8>) -> Self {
        Self {
            filter: OwnerFilter::new(owner),
            limit: None,
            last_id: None,
        }
    }
}

impl<ID: DocumentId> From<PageBuilder<ID, OwnerFilter>> for sdk::ListAccountMetadataRequest {
    fn from(builder: PageBuilder<ID, OwnerFilter>) -> Self {
        let page = PageBuilder::<ID, ()> {
            filter: (),
            limit: builder.limit,
            last_id: builder.last_id,
        }
        .into();
        Self {
            page,
            filter: Some(builder.filter.into()),
        }
    }
}
