use bytes::Bytes;
use reqwest::{Client, multipart::Form};
use serde::Deserialize;

use crate::{context::{API_URL, Context}, util::{DataList, FileResource}};

#[derive(Debug, Deserialize)]
pub struct FileInfo {
    pub id: String,
    /* pub object: "file", */
    pub bytes: u64,
    pub created_at: u64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct FileDeleteResponse {
    pub id: String,
    /* pub object: "file", */
    pub deleted: bool,
}

impl Context {
    pub async fn get_files(&self) -> anyhow::Result<Vec<FileInfo>> {
        Ok(
            self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/files")))
                .send()
                .await?
                .error_for_status()?
                .json::<DataList<FileInfo>>()
                .await?
                .data
        )
    }

    pub async fn upload_file(&self, file: FileResource, file_name: String, purpose: String) -> anyhow::Result<FileInfo> {
        Ok(
            self.with_auth(Client::builder().build()?.post(&format!("{API_URL}/v1/files")))
                .multipart(file.write_file_named(Form::new().text("purpose", purpose), "file", file_name))
                .send()
                .await?
                .error_for_status()?
                .json::<FileInfo>()
                .await?
        )
    }

    pub async fn delete_file(&self, file_id: &str) -> anyhow::Result<FileDeleteResponse> {
        Ok(
            self.with_auth(Client::builder().build()?.delete(&format!("{API_URL}/v1/files/{file_id}")))
                .send()
                .await?
                .error_for_status()?
                .json::<FileDeleteResponse>()
                .await?
        )
    }

    pub async fn get_file(&self, file_id: &str) -> anyhow::Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>> {
        Ok(
            self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/files/{file_id}")))
                .send()
                .await?
                .error_for_status()?
                .bytes_stream()
        )
    }

    pub async fn get_file_direct(&self, file_id: &str) -> anyhow::Result<Bytes> {
        Ok(
            self.with_auth(Client::builder().build()?.get(&format!("{API_URL}/v1/files/{file_id}")))
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?
        )
    }
}