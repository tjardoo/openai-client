use crate::v1::endpoints::administration::Administration;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::administration::invite::CreateInviteParameters;
use crate::v1::resources::administration::invite::Invite;
use crate::v1::resources::shared::DeletedObject;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::shared::SimpleListParameters;

pub struct Invites<'a> {
    pub administration: &'a Administration<'a>,
}

impl Administration<'_> {
    /// Invite and manage invitations for an organization.
    pub fn invites(&self) -> Invites<'_> {
        Invites {
            administration: self,
        }
    }
}

impl Invites<'_> {
    /// Returns a list of invites in the organization.
    pub async fn list(
        &self,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<Invite>, APIError> {
        let response = self
            .administration
            .client
            .get_with_query("/organization/invites", &query)
            .await?;

        let response: ListResponse<Invite> = format_response(response)?;

        Ok(response)
    }

    // Retrieves an invite.
    pub async fn retrieve(&self, invite_id: &str) -> Result<Invite, APIError> {
        let response = self
            .administration
            .client
            .get(&format!("/organization/invites/{invite_id}"))
            .await?;

        let response: Invite = format_response(response)?;

        Ok(response)
    }

    /// Create an invite for a user to the organization.
    pub async fn create(&self, parameters: CreateInviteParameters) -> Result<Invite, APIError> {
        let response = self
            .administration
            .client
            .post("/organization/invites", &parameters, None)
            .await?;

        let response: Invite = format_response(response.data)?;

        Ok(response)
    }

    /// Delete an invite.
    pub async fn delete(&self, invite_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .administration
            .client
            .delete(&format!("/organization/invites/{invite_id}"))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
