use derive_builder::Builder;
use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::{completion::Usage, context::{API_URL, Context}};

#[derive(Debug, Serialize, Builder)]
pub struct EditRequest {
    #[builder(setter(into))]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input: Option<String>,
    #[builder(setter(into))]
    pub instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub n: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Edit {
    pub text: String,
    pub index: i32,
}

#[derive(Debug, Deserialize)]
pub struct EditResponse {
    /* pub object: "edit", */
    pub created: u64,
    pub choices: Vec<Edit>,
    pub usage: Usage
}

impl Context {
    pub async fn create_edit(&self, edit_request: EditRequest) -> anyhow::Result<EditResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/edits")))
                .json(&edit_request)
                .send()
                .await?
                .error_for_status()?
                .json::<EditResponse>()
                .await?
        )
    }
}