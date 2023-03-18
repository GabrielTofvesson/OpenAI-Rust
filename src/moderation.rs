use derive_builder::Builder;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{completion::Sequence, context::{API_URL, Context}};

#[derive(Debug, Serialize, Builder)]
pub struct ModerationRequest {
    #[builder(setter(into))]
    pub input: Sequence,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub model: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Categories<T> {
    pub hate: T,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: T,
    #[serde(rename = "self-harm")]
    pub self_harm: T,
    pub sexual: T,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: T,
    pub violence: T,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: T,
}

#[derive(Debug, Deserialize)]
pub struct Moderation {
    pub categories: Categories<bool>,
    pub category_scores: Categories<f64>,
    pub flagged: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<Moderation>,
}

impl Context {
    pub async fn create_moderation(&self, moderation_request: ModerationRequest) -> anyhow::Result<ModerationResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/moderations")))
                .json(&moderation_request)
                .send()
                .await?
                .error_for_status()?
                .json::<ModerationResponse>()
                .await?
        )
    }
}