use derive_builder::Builder;
use reqwest::{multipart::{Form, Part}, Body, Client};
use tokio_util::codec::{FramedRead, BytesCodec};

use crate::{context::{API_URL, Context}, transcription::TranscriptionResponse};
use crate::transcription::{AudioFile, AudioResponseFormat};

type TranslationResponse = TranscriptionResponse;

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct TranslationRequest {
    #[builder(setter(into))]
    pub file: AudioFile,
    #[builder(setter(into))]
    pub model: String,
    #[builder(setter(into, strip_option), default)]
    pub prompt: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub response_format: Option<AudioResponseFormat>,
    #[builder(setter(into, strip_option), default)]
    pub temperature: Option<f64>,
}

impl Context {
    pub async fn create_translation(&self, req: TranslationRequest) -> anyhow::Result<TranslationResponse> {
        let mut form = Form::new();
        let file_name = req.file.file_name();
        form = form.part("file", Part::stream(Body::wrap_stream(FramedRead::new(req.file.file(), BytesCodec::new()))).file_name(file_name));
        form = form.text("model", req.model);

        if let Some(response_format) = req.response_format {
            form = form.text("response_format", response_format.to_string());
        }
        if let Some(prompt) = req.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(temperature) = req.temperature {
            form = form.text("temperature", temperature.to_string());
        }
        
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/audio/translations")).multipart(form)).send().await?.json::<TranslationResponse>().await?)
    }
}