use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Project {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always organization.project.
    pub object: String,
    /// The name of the project. This appears in reporting.
    pub name: String,
    /// The Unix timestamp (in seconds) of when the project was created.
    pub created_at: u32,
    /// The Unix timestamp (in seconds) of when the project was archived or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<u32>,
    /// The status of the project.
    pub status: ProjectStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Active,
    Archived,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq)]
#[builder(name = "CreateProjectParametersBuilder")]
#[builder(setter(into, strip_option))]
pub struct CreateProjectParameters {
    /// The friendly name of the project, this name appears in reports.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq)]
#[builder(name = "ModifyProjectParametersBuilder")]
#[builder(setter(into, strip_option))]
pub struct ModifyProjectParameters {
    /// The updated name of the project, this name appears in reports.
    pub name: String,
}
