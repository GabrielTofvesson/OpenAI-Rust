use derive_builder::Builder;
use reqwest::{multipart::Form, Client};

use crate::{image_edit::{ImageFile, write_file}, image::{ImageSize, ResponseFormat, ImageResponse}, context::{API_URL, Context}};

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct ImageVariationRequest {
    #[builder(setter(into))]
    pub image: ImageFile,
    #[builder(setter(into, strip_option), default)]
    pub n: Option<u32>,
    #[builder(setter(into, strip_option), default)]
    pub size: Option<ImageSize>,
    #[builder(setter(into, strip_option), default)]
    pub user: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub response_format: Option<ResponseFormat>,
}


impl Context {
    pub async fn create_image_variation(&self, req: ImageVariationRequest) -> anyhow::Result<ImageResponse> {
        let mut form = Form::new();
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

        if let Some(size) = req.size {
            form = form.text("size", size.to_string());
        }
        
        Ok(self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/images/variations")).multipart(form)).send().await?.json::<ImageResponse>().await?)
    }
}