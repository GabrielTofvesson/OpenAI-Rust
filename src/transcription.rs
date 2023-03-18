use derive_builder::Builder;
use reqwest::{multipart::{Form, Part}, Body, Client};
use serde::Deserialize;
use tokio::fs::File;
use tokio_util::codec::{FramedRead, BytesCodec};

use crate::context::{API_URL, Context};

#[derive(Debug, Clone)]
pub enum AudioResponseFormat {
    Text,
    Json,
    Srt,
    Vtt,
    VerboseJson,
}

#[derive(Debug)]
pub enum AudioFile {
    MP3(File),
    MP4(File),
    MPEG(File),
    MPGA(File),
    WAV(File),
    WEBM(File),
}

impl AudioFile {
    fn file_name(&self) -> &'static str {
        match self {
            AudioFile::MP3(_) => "file.mp3",
            AudioFile::MP4(_) => "file.mp4",
            AudioFile::MPEG(_) => "file.mpeg",
            AudioFile::MPGA(_) => "file.mpga",
            AudioFile::WAV(_) => "file.wav",
            AudioFile::WEBM(_) => "file.webm",
        }
    }

    fn file(self) -> File {
        match self {
            AudioFile::MP3(file) => file,
            AudioFile::MP4(file) => file,
            AudioFile::MPEG(file) => file,
            AudioFile::MPGA(file) => file,
            AudioFile::WAV(file) => file,
            AudioFile::WEBM(file) => file,
        }
    }
}

impl ToString for AudioResponseFormat {
    fn to_string(&self) -> String {
        match self {
            AudioResponseFormat::Text => "text",
            AudioResponseFormat::Json => "json",
            AudioResponseFormat::Srt => "srt",
            AudioResponseFormat::Vtt => "vtt",
            AudioResponseFormat::VerboseJson => "verbose_json",
        }.to_string()
    }
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct TranscriptionRequest {
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
    #[builder(setter(into, strip_option), default)]
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
}

impl Context {
    pub async fn create_transcription(&self, req: TranscriptionRequest) -> anyhow::Result<TranscriptionResponse> {
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

        if let Some(language) = req.language {
            form = form.text("language", language.to_string());
        }
        
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/audio/transcriptions")).multipart(form)).send().await?.json::<TranscriptionResponse>().await?)
    }
}