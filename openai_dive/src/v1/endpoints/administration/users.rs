use crate::v1::endpoints::administration::Administration;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::administration::user::ModifyUserParameters;
use crate::v1::resources::administration::user::User;
use crate::v1::resources::shared::DeletedObject;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::shared::SimpleListParameters;

pub struct Users<'a> {
    pub administration: &'a Administration<'a>,
}

impl Administration<'_> {
    /// Manage users and their role in an organization.
    pub fn users(&self) -> Users<'_> {
        Users {
            administration: self,
        }
    }
}

impl Users<'_> {
    /// Lists all of the users in the organization.
    pub async fn list(
        &self,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<User>, APIError> {
        let response = self
            .administration
            .client
            .get_with_query("/organization/users", &query)
            .await?;

        let response: ListResponse<User> = format_response(response)?;

        Ok(response)
    }

    /// Retrieves a user by their identifier.
    pub async fn retrieve(&self, user_id: &str) -> Result<User, APIError> {
        let response = self
            .administration
            .client
            .get(&format!("/organization/users/{user_id}"))
            .await?;

        let response: User = format_response(response)?;

        Ok(response)
    }

    /// Modifies a user's role in the organization.
    pub async fn modify(
        &self,
        user_id: &str,
        parameters: ModifyUserParameters,
    ) -> Result<User, APIError> {
        let response = self
            .administration
            .client
            .post(&format!("/organization/users/{user_id}"), &parameters, None)
            .await?;

        let response: User = format_response(response.data)?;

        Ok(response)
    }

    /// Deletes a user from the organization.
    pub async fn delete(&self, user_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .administration
            .client
            .delete(&format!("/organization/users/{user_id}"))
            .await?;

        let response: DeletedObject = format_response(response)?;

        Ok(response)
    }
}
