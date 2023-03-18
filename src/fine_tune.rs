use derive_builder::Builder;
use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::{file::FileInfo, context::{API_URL, Context}, util::DataList};

#[derive(Debug, Serialize, Builder)]
pub struct CreateFineTuneRequest {
    #[builder(setter(into))]
    pub training_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub validation_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub n_epochs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub batch_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub learning_rate_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub prompt_loss_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub compute_classification_metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub classification_n_classes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub classification_positive_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub classification_betas: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub suffix: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FineTuneEvent {
    /* pub object: "fine-tune-event", */
    pub created_at: u64,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Hyperparams {
    pub batch_size: u32,
    pub learning_rate_multiplier: f64,
    pub prompt_loss_weight: f64,
    pub n_epochs: u32,
}

#[derive(Debug)]
pub enum FineTuneStatus {
    Pending,
    Succeeded,
    Cancelled,
}

impl<'de> Deserialize<'de> for FineTuneStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "pending" => Ok(FineTuneStatus::Pending),
            "succeeded" => Ok(FineTuneStatus::Succeeded),
            "cancelled" => Ok(FineTuneStatus::Cancelled),
            _ => Err(serde::de::Error::custom(format!("Invalid fine-tune status: {}", s))),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FineTuneResponse {
    pub id: String,
    /* pub object: "fine-tune", */
    pub model: String,
    pub created_at: u64,
    pub events: Vec<FineTuneEvent>,
    pub fine_tuned_model: Option<String>,
    pub hyperparams: Hyperparams,
    pub organization_id: String,
    pub result_files: Vec<FileInfo>,
    pub status: FineTuneStatus,
    pub validation_files: Vec<FileInfo>,
    pub training_files: Vec<FileInfo>,
    pub updated_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct FineTuneDeleteResponse {
    pub id: String,
    /* pub object: "fine-tune", */
    pub deleted: bool,
}

impl Context {
    pub async fn create_fine_tune(&self, request: CreateFineTuneRequest) -> anyhow::Result<FineTuneResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/fine-tunes")))
                .json(&request)
                .send()
                .await?
                .error_for_status()?
                .json::<FineTuneResponse>()
                .await?
        )
    }

    pub async fn get_fine_tune(&self, id: impl Into<String>) -> anyhow::Result<FineTuneResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/fine-tunes/{}", id.into())))
                .send()
                .await?
                .error_for_status()?
                .json::<FineTuneResponse>()
                .await?
        )
    }
    
    pub async fn list_fine_tunes(&self) -> anyhow::Result<Vec<FineTuneResponse>> {
        Ok(
            self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/fine-tunes")))
                .send()
                .await?
                .error_for_status()?
                .json::<DataList<FineTuneResponse>>()
                .await?
                .data
        )
    }

    pub async fn cancel_fine_tune(&self, id: impl Into<String>) -> anyhow::Result<FineTuneResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.delete(&format!("{API_URL}/v1/fine-tunes/{}", id.into())))
                .send()
                .await?
                .error_for_status()?
                .json::<FineTuneResponse>()
                .await?
        )
    }
    
    pub async fn list_fine_tune_events(&self, id: impl Into<String>) -> anyhow::Result<Vec<FineTuneEvent>> {
        Ok(
            self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/fine-tunes/{}/events", id.into())))
                .send()
                .await?
                .error_for_status()?
                .json::<DataList<FineTuneEvent>>()
                .await?
                .data
        )
    }

    pub async fn delete_fine_tune(&self, id: impl Into<String>) -> anyhow::Result<FineTuneDeleteResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.delete(&format!("{API_URL}/v1/fine-tunes/{}", id.into())))
                .send()
                .await?
                .error_for_status()?
                .json::<FineTuneDeleteResponse>()
                .await?
        )
    }
}