use base64::{Engine, prelude::BASE64_STANDARD};
use derive_builder::Builder;
use reqwest::{Body, multipart::{Part, Form}, Client};
use crate::{image::{ResponseFormat, ImageResponse}, context::{API_URL, Context}};
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Debug)]
pub enum ImageFile {
    File(tokio::fs::File),
    Data(Vec<u8>),
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct ImageEditRequest {
    #[builder(setter(into))]
    pub image: ImageFile,
    #[builder(setter(into, strip_option), default)]
    pub mask: Option<ImageFile>,
    #[builder(setter(into))]
    pub prompt: String,
    #[builder(setter(into, strip_option), default)]
    pub n: Option<u32>,
    #[builder(setter(into, strip_option), default)]
    pub response_format: Option<ResponseFormat>,
    #[builder(setter(into, strip_option), default)]
    pub user: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub temperature: Option<f64>,
}

fn write_file(form: Form, file: ImageFile, name: impl Into<String>) -> Form {
    let name = name.into();
    match file {
        ImageFile::File(file) =>
            form.part(name.clone(), Part::stream(Body::wrap_stream(FramedRead::new(file, BytesCodec::new()))).file_name(name)),
        ImageFile::Data(data) =>
            form.text(name, BASE64_STANDARD.encode(data.as_slice())),
    }
}

impl Context {
    pub async fn create_image_edit(&self, req: ImageEditRequest) -> anyhow::Result<ImageResponse> {
        let mut form = Form::new();
        form = form.text("prompt", req.prompt);
        form = write_file(form, req.image, "image");

        if let Some(n) = req.n {
            form = form.text("n", n.to_string());
        }
        if let Some(response_format) = req.response_format {
            form = form.text("response_format", response_format.to_string());
        }
        if let Some(user) = req.user {
            form = form.text("user", user);
        }
        
        if let Some(mask) = req.mask {
            form = write_file(form, mask, "mask");
        }

        if let Some(temperature) = req.temperature {
            form = form.text("temperature", temperature.to_string());
        }
        
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/images/edits")).multipart(form)).send().await?.json::<ImageResponse>().await?)
    }
}