use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Permission {
    pub id: String,
    /* pub object: "model_permission", */
    pub created: u64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    /* pub group: null, */
    pub is_blocking: bool,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
    pub created: u64,
    pub owned_by: String,
    pub permission: Vec<Permission>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ModelList {
    pub data: Vec<Model>,
}