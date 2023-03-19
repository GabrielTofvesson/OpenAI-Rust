use std::{collections::HashMap, str::FromStr, pin::Pin, task::Poll};

use derive_builder::Builder;
use futures::{Stream, StreamExt};
use reqwest::{Client, RequestBuilder};
use reqwest_eventsource::{RequestBuilderExt, Event, EventSource};
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
#[builder(pattern = "owned")]
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
    stream: Option<bool>,
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

#[derive(Debug)]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
}

impl<'de> Deserialize<'de> for FinishReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            // Deserialize the String
            match String::deserialize(deserializer)? {
                s if s == "stop" => Ok(Self::Stop),
                s if s == "length" => Ok(Self::Length),
                s if s == "content_filter" => Ok(Self::ContentFilter),
                _ => Err(serde::de::Error::custom("Invalid stop reason")),
            }

    }
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletion {
    pub index: i32,
    pub message: ChatMessage,
    pub finish_reason: Option<FinishReason>
}

#[derive(Debug, Deserialize)]
pub struct DeltaMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeltaChatCompletion {
    pub index: i32,
    pub delta: DeltaMessage,
    pub finish_reason: Option<FinishReason>,
}
#[derive(Debug, Deserialize)]
pub struct ChatCompletionDeltaResponse {
    pub id: String,
    /* pub object: "chat.completion", */
    pub created: u64,
    pub model: String,
    pub choices: Vec<DeltaChatCompletion>,
}

impl FromStr for ChatCompletionDeltaResponse {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionSyncResponse {
    pub id: String,
    /* pub object: "chat.completion", */
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletion>,
    pub usage: Usage
}

struct CompletionStream {
    stream: EventSource
}

impl Stream for CompletionStream {
    type Item = anyhow::Result<ChatCompletionDeltaResponse>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            return match self.stream.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(event))) => {
                    match event {
                        Event::Message(message) => {
                            // Stream has ended
                            if message.data == "[DONE]" {
                                return Poll::Ready(None)
                            }

                            match message.data.parse::<ChatCompletionDeltaResponse>() {
                                Ok(value) => Poll::Ready(Some(Ok(value))),
                                Err(e) => Poll::Ready(Some(Err(e.into())))
                            }
                        },
                        _ => continue
                    }
                },
                Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(anyhow::Error::new(e)))),
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending
            }
        }
    }
}

impl Context {
    fn build_request(&self, stream: bool, chat_completion_request: ChatHistoryBuilder) -> anyhow::Result<RequestBuilder> {
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/chat/completions")))
            .json(&chat_completion_request.stream(stream).build()?))
    }

    pub async fn create_chat_completion_sync(&self, chat_completion_request: ChatHistoryBuilder) -> anyhow::Result<ChatCompletionSyncResponse> {
        Ok(
            self.build_request(false, chat_completion_request)?
                .send()
                .await?
                .error_for_status()?
                .json::<ChatCompletionSyncResponse>()
                .await?
        )
    }

    pub async fn create_chat_completion_streamed(&self, chat_completion_request: ChatHistoryBuilder) -> anyhow::Result<impl Stream<Item = anyhow::Result<ChatCompletionDeltaResponse>>> {
        Ok(CompletionStream { stream: self.build_request(true, chat_completion_request)?.eventsource()? })
    }
}