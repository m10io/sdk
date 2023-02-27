use crate::types::PublicKey;
use crate::{DocumentId, OwnerFilter};
use m10_protos::sdk;

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

impl From<NameOrOwnerFilter> for sdk::list_account_metadata_request::Filter {
    fn from(filter: NameOrOwnerFilter) -> Self {
        match filter {
            NameOrOwnerFilter::Name(name) => sdk::list_account_metadata_request::Filter::Name(name),
            NameOrOwnerFilter::Owner(owner) => {
                sdk::list_account_metadata_request::Filter::Owner(owner.0)
            }
        }
    }
}

impl<ID: DocumentId> From<PageBuilder<ID, NameFilter>> for sdk::ListRolesRequest {
    fn from(builder: PageBuilder<ID, NameFilter>) -> Self {
        let page = PageBuilder::<ID, ()> {
            filter: (),
            limit: builder.limit,
            last_id: builder.last_id,
        }
        .into();
        Self {
            page,
            filter: Some(sdk::list_roles_request::Filter::Name(builder.filter.name)),
        }
    }
}

impl<ID: DocumentId> From<PageBuilder<ID, NameFilter>> for sdk::ListRoleBindingsRequest {
    fn from(builder: PageBuilder<ID, NameFilter>) -> Self {
        let page = PageBuilder::<ID, ()> {
            filter: (),
            limit: builder.limit,
            last_id: builder.last_id,
        }
        .into();
        Self {
            page,
            filter: Some(sdk::list_role_bindings_request::Filter::Name(
                builder.filter.name,
            )),
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

impl<ID: DocumentId> From<PageBuilder<ID, NameOrOwnerFilter>> for sdk::ListAccountMetadataRequest {
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
