pub mod context;
pub mod model;
pub mod completion;
pub mod chat;

#[cfg(test)]
mod tests {
    use crate::chat::ChatMessage;
    use crate::context::Context;
    use crate::completion::CompletionRequestBuilder;

    fn get_api() -> anyhow::Result<Context> {
        Ok(Context::new(std::fs::read_to_string(std::path::Path::new("apikey.txt"))?.trim().to_string()))
    }


    #[tokio::test]
    async fn test_get_models() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");

        let models = ctx.unwrap().get_models().await;
        assert!(models.is_ok(), "Could not get models: {}", models.unwrap_err());
        assert!(models.unwrap().len() > 0, "No models found");
    }

    #[tokio::test]
    async fn test_completion() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");

        let completion = ctx.unwrap().create_completion(
            CompletionRequestBuilder::default()
            .model("text-davinci-003")
            .prompt("Say 'this is a test'")
            .build()
            .unwrap()
        ).await;
        
        assert!(completion.is_ok(), "Could not get completion: {}", completion.unwrap_err());
        assert!(completion.unwrap().choices.len() == 1, "No completion found");
    }

    #[tokio::test]
    async fn test_chat_completion() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");

        let completion = ctx.unwrap().create_chat_completion(
            crate::chat::ChatHistoryBuilder::default()
            .messages(vec![ChatMessage::new(crate::chat::Role::User, "Respond to this message with 'this is a test'")])
            .model("gpt-3.5-turbo")
            .build()
            .unwrap()
        ).await;

        assert!(completion.is_ok(), "Could not get completion: {}", completion.unwrap_err());
        assert!(completion.unwrap().choices.len() == 1, "No completion found");
    }
}