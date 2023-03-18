use std::collections::HashMap;

use derive_builder::Builder;
use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::context::{API_URL, Context};

#[derive(Debug, Clone)]
pub enum Sequence {
    String(String),
    List(Vec<String>),
}

impl Serialize for Sequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Sequence::String(s) => serializer.serialize_str(s),
            Sequence::List(l) => serializer.collect_seq(l),
        }
    }
}

impl From<String> for Sequence {
    fn from(s: String) -> Self {
        Sequence::String(s)
    }
}

impl From<Vec<String>> for Sequence {
    fn from(v: Vec<String>) -> Self {
        Sequence::List(v)
    }
}

impl From<Vec<&str>> for Sequence {
    fn from(v: Vec<&str>) -> Self {
        Sequence::List(v.iter().map(|s| s.to_string()).collect())
    }
}

impl From<&str> for Sequence {
    fn from(s: &str) -> Self {
        Sequence::String(s.to_string())
    }
}

impl From<&[&str]> for Sequence {
    fn from(v: &[&str]) -> Self {
        Sequence::List(v.iter().map(|s| s.to_string()).collect())
    }
}

#[derive(Debug, Serialize, Builder)]
pub struct CompletionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub prompt: Option<Sequence>,
    #[builder(setter(into))]
    pub model: String,
    #[builder(setter(into, strip_option), default)]
    pub max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub n: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub logprobs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub stop: Option<Sequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub best_of: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub logit_bias: Option<HashMap<u64, i8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub return_prompt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u64,
    pub text: String,
    pub logprobs: Option<HashMap<String, f64>>,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl Context {
    pub async fn create_completion(&self, completion_request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/completions")).json(&completion_request)).send().await?.json::<CompletionResponse>().await?)
    }
}