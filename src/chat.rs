use std::collections::HashMap;

use derive_builder::Builder;
use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::{completion::{Sequence, Usage}, context::{API_URL, Context}};

#[derive(Debug, Clone)]
pub enum Role {
    User,
    System,
    Assistant
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::System => serializer.serialize_str("system"),
            Self::Assistant => serializer.serialize_str("assistant"),
        }
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            // Deserialize the String
            match String::deserialize(deserializer)? {
                s if s == "user" => Ok(Self::User),
                s if s == "system" => Ok(Self::System),
                s if s == "assistant" => Ok(Self::Assistant),
                _ => Err(serde::de::Error::custom("Invalid role")),
            }

    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: Role, message: impl Into<String>) -> Self {
        Self {
            role,
            content: message.into()
        }
    }
}

#[derive(Debug, Serialize, Builder)]
pub struct ChatHistory {
    #[builder(setter(into))]
    pub messages: Vec<ChatMessage>,
    #[builder(setter(into))]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub stop: Option<Sequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub logit_bias: Option<HashMap<u64, i8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletion {
    pub index: i32,
    pub message: ChatMessage,
    pub finish_reason: String, // TODO: Create enum for this
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    /* pub object: "chat.completion", */
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletion>,
    pub usage: Usage
}

impl Context {
    pub async fn create_chat_completion(&self, chat_completion_request: ChatHistory) -> anyhow::Result<ChatCompletionResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/chat/completions")))
                .json(&chat_completion_request)
                .send()
                .await?
                .error_for_status()?
                .json::<ChatCompletionResponse>()
                .await?
        )
    }
}