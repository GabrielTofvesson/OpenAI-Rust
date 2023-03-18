use derive_builder::Builder;
use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::context::{API_URL, Context};

#[derive(Debug, Clone)]
pub enum ResponseFormat {
    URL,
    Base64,
}

impl ToString for ResponseFormat {
    fn to_string(&self) -> String {
        match self {
            Self::URL => "url".to_string(),
            Self::Base64 => "b64_json".to_string(),
        }
    }
}

impl Serialize for ResponseFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum ImageSize {
    Size256,
    Size512,
    Size1024,
}

impl ToString for ImageSize {
    fn to_string(&self) -> String {
        match self {
            Self::Size256 => "256x256".to_string(),
            Self::Size512 => "512x512".to_string(),
            Self::Size1024 => "1024x1024".to_string(),
        }
    }
}

impl Serialize for ImageSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Serialize, Builder)]
pub struct ImageRequest {
    #[builder(setter(into))]
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub size: Option<ImageSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub temperature: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct RawImage {
    pub(crate) url: Option<String>,
    pub(crate) b64_json: Option<String>,
}

#[derive(Debug)]
pub enum Image {
    URL(String),
    Base64(String),
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let raw = RawImage::deserialize(deserializer)?;
        match (raw.url, raw.b64_json) {
            (Some(url), None) => Ok(Self::URL(url)),
            (None, Some(b64)) => Ok(Self::Base64(b64)),
            _ => Err(serde::de::Error::custom("Invalid image")),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageResponse {
    pub created: u64,
    pub data: Vec<Image>,
}

impl Context {
    pub async fn create_image(&self, image_request: ImageRequest) -> anyhow::Result<ImageResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/images/generations")))
                .json(&image_request)
                .send()
                .await?
                .error_for_status()?
                .json::<ImageResponse>()
                .await?
        )
    }
}