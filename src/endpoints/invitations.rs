use crate::client::FleetClient;
use crate::error::Result;
use crate::models::invitation::*;
use crate::paths;

/// Endpoint for managing user invitations
pub struct InvitationsEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> InvitationsEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// List invitations with optional search
    pub fn list(&self) -> ListInvitationsBuilder<'_> {
        ListInvitationsBuilder {
            client: self.client,
            order_key: None,
            order_direction: None,
            query: None,
        }
    }

    /// Create a new invitation
    pub async fn create(&self, req: CreateInvitationRequest) -> Result<CreateInvitationResponse> {
        req.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, paths::INVITES)?
            .json(&req);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get an invitation by ID
    pub async fn get(&self, invite_id: u64) -> Result<GetInvitationResponse> {
        let path = paths::invite(invite_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Update an invitation
    pub async fn update(
        &self,
        invite_id: u64,
        req: UpdateInvitationRequest,
    ) -> Result<UpdateInvitationResponse> {
        req.validate()?;
        let path = paths::invite(invite_id);
        let request = self
            .client
            .request(reqwest::Method::PATCH, &path)?
            .json(&req);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Delete an invitation
    pub async fn delete(&self, invite_id: u64) -> Result<()> {
        let path = paths::invite(invite_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }
}

/// Builder for listing invitations
pub struct ListInvitationsBuilder<'a> {
    client: &'a FleetClient,
    order_key: Option<String>,
    order_direction: Option<String>,
    query: Option<String>,
}

impl<'a> ListInvitationsBuilder<'a> {
    pub fn order_key(mut self, key: impl Into<String>) -> Self {
        self.order_key = Some(key.into());
        self
    }

    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.order_direction = Some(direction.to_string());
        self
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Execute the list invitations query
    pub async fn send(self) -> Result<ListInvitationsResponse> {
        crate::models::common::validate_list_options(
            None,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            false,
        )?;
        let mut params = vec![];
        if let Some(ref key) = self.order_key {
            params.push(("order_key".to_string(), key.clone()));
        }
        if let Some(ref dir) = self.order_direction {
            params.push(("order_direction".to_string(), dir.clone()));
        }
        if let Some(ref q) = self.query {
            params.push(("query".to_string(), q.clone()));
        }

        let path = crate::http::append_query_params(paths::INVITES, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
