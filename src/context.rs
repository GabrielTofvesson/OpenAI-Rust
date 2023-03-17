use reqwest::{Client, RequestBuilder};

use crate::{model::{Model, ModelList}, completion::{CompletionRequest, CompletionResponse}, chat::{ChatCompletionResponse, ChatHistory}, edits::{EditRequest, EditResponse}};

pub struct Context {
    api_key: String,
    org_id: Option<String>
}

const API_URL: &str = "https://api.openai.com";

impl Context {
    pub fn new(api_key: String) -> Self {
        Context {
            api_key,
            org_id: None,
        }
    }

    pub fn new_with_org(api_key: String, org_id: String) -> Self {
        Context {
            api_key,
            org_id: Some(org_id),
        }
    }

    fn with_auth(&self, builder: RequestBuilder) -> RequestBuilder {
        (
            if let Some(ref org_id) = self.org_id {
                builder.header("OpenAI-Organization", org_id)
            } else {
                builder
            }
        ).bearer_auth(&self.api_key)
    }

    pub async fn get_models(&self) -> anyhow::Result<Vec<Model>> {
        Ok(self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/models"))).send().await?.json::<ModelList>().await?.data)
    }

    pub async fn get_model(&self, model_id: &str) -> anyhow::Result<Model> {
        Ok(self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/models/{model_id}", model_id = model_id))).send().await?.json::<Model>().await?)
    }

    pub async fn create_completion(&self, completion_request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/completions")).json(&completion_request)).send().await?.json::<CompletionResponse>().await?)
    }

    pub async fn create_chat_completion(&self, chat_completion_request: ChatHistory) -> anyhow::Result<ChatCompletionResponse> {
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/chat/completions")).json(&chat_completion_request)).send().await?.json::<ChatCompletionResponse>().await?)
    }

    pub async fn create_edit(&self, edit_request: EditRequest) -> anyhow::Result<crate::edits::EditResponse> {
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/edits")).json(&edit_request)).send().await?.json::<EditResponse>().await?)
    }

    pub async fn create_image(&self, image_request: crate::image::ImageRequest) -> anyhow::Result<crate::image::ImageResponse> {
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/images/generations")).json(&image_request)).send().await?.json::<crate::image::ImageResponse>().await?)
    }
}