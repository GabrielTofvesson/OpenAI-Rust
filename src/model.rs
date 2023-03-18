use reqwest::Client;
use serde::Deserialize;

use crate::{context::{API_URL, Context}, util::DataList};

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

impl Context {
    pub async fn get_models(&self) -> anyhow::Result<Vec<Model>> {
        Ok(self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/models"))).send().await?.json::<DataList<Model>>().await?.data)
    }

    pub async fn get_model(&self, model_id: &str) -> anyhow::Result<Model> {
        Ok(self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/models/{model_id}", model_id = model_id))).send().await?.json::<Model>().await?)
    }
}