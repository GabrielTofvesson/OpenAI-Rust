use derive_builder::Builder;
use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::{completion::Sequence, context::{API_URL, Context}};

#[derive(Debug, Serialize, Builder)]
pub struct EmbeddingRequest {
    #[builder(setter(into))]
    pub model: String,
    #[builder(setter(into))]
    pub input: Sequence,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Embedding {
    /* pub object: "embedding", */
    pub embedding: Vec<f64>,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse {
    /* pub object: "list", */
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

impl Context {
    pub async fn create_embedding(&self, embedding_request: EmbeddingRequest) -> anyhow::Result<EmbeddingResponse> {
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/embeddings")).json(&embedding_request)).send().await?.json::<EmbeddingResponse>().await?)
    }
}