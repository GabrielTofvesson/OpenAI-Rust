use reqwest::RequestBuilder;

pub struct Context {
    api_key: String,
    org_id: Option<String>
}

pub(crate) const API_URL: &str = "https://api.openai.com";

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

    pub(crate) fn with_auth(&self, builder: RequestBuilder) -> RequestBuilder {
        (
            if let Some(ref org_id) = self.org_id {
                builder.header("OpenAI-Organization", org_id)
            } else {
                builder
            }
        ).bearer_auth(&self.api_key)
    }
}