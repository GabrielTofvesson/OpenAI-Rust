use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{multipart::{Form, Part}, Body};
use serde::Deserialize;
use tokio_util::codec::{FramedRead, BytesCodec};

#[derive(Debug, Deserialize)]
pub struct DataList<T> {
    pub data: Vec<T>,
    /* pub object: "list", */
}


#[derive(Debug)]
pub enum FileResource {
    File(tokio::fs::File),
    Data(Vec<u8>),
}

impl FileResource {
    pub(crate) fn write_file_named(self, form: Form, part_name: impl Into<String>, file_name: impl Into<String>) -> Form {
        match self {
            FileResource::File(file) =>
                form.part(part_name.into(), Part::stream(Body::wrap_stream(FramedRead::new(file, BytesCodec::new()))).file_name(file_name.into())),
            FileResource::Data(data) =>
                form.part(part_name.into(), Part::bytes(BASE64_STANDARD.encode(data.as_slice()).as_bytes().to_owned()).file_name(file_name.into())),
        }
    }

    pub(crate) fn write_file(self, form: Form, name: impl Into<String>) -> Form {
        let name = name.into();
        self.write_file_named(form, name.clone(), name)
    }
    
}

impl From<tokio::fs::File> for FileResource {
    fn from(file: tokio::fs::File) -> Self {
        Self::File(file)
    }
}

impl From<Vec<u8>> for FileResource {
    fn from(data: Vec<u8>) -> Self {
        Self::Data(data)
    }
}