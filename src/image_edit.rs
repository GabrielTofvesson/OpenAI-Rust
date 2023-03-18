use derive_builder::Builder;
use reqwest::{multipart::Form, Client};
use crate::{image::{ResponseFormat, ImageResponse, ImageSize}, context::{API_URL, Context}, util::FileResource};

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct ImageEditRequest {
    #[builder(setter(into))]
    pub image: FileResource,
    #[builder(setter(into, strip_option), default)]
    pub mask: Option<FileResource>,
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
    #[builder(setter(into, strip_option), default)]
    pub size: Option<ImageSize>,
}

impl Context {
    pub async fn create_image_edit(&self, req: ImageEditRequest) -> anyhow::Result<ImageResponse> {
        let mut form = Form::new();
        form = form.text("prompt", req.prompt);
        form = req.image.write_file(form, "image");

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
            form = mask.write_file(form, "mask");
        }

        if let Some(temperature) = req.temperature {
            form = form.text("temperature", temperature.to_string());
        }

        if let Some(size) = req.size {
            form = form.text("size", size.to_string());
        }
        
        Ok(
            self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/images/edits")))
                .multipart(form)
                .send()
                .await?
                .error_for_status()?
                .json::<ImageResponse>()
                .await?
        )
    }
}