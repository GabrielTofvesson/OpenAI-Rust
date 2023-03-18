use derive_builder::Builder;
use serde::{Serialize, Deserialize};

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

impl Serialize for ImageSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Self::Size256 => serializer.serialize_str("256x256"),
            Self::Size512 => serializer.serialize_str("512x512"),
            Self::Size1024 => serializer.serialize_str("1024x1024"),
        }
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