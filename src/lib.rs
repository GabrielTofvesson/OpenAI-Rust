pub mod context;
pub mod model;
pub mod completion;
pub mod chat;
pub mod edits;
pub mod image;

#[cfg(test)]
mod tests {
    use crate::chat::ChatMessage;
    use crate::context::Context;
    use crate::completion::CompletionRequestBuilder;
    use crate::image::Image;

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

    #[tokio::test]
    async fn test_edits() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");
        let ctx = ctx.unwrap();

        // Autocorrect English spelling errors
        let edit = ctx.create_edit(
            crate::edits::EditRequestBuilder::default()
            .model("text-davinci-edit-001")
            .instruction("Correct all spelling mistakes")
            .input("What a wnoderful day!")
            .build()
            .unwrap()
        ).await;

        assert!(edit.is_ok(), "Could not get edit: {}", edit.unwrap_err());
        assert!(edit.as_ref().unwrap().choices.len() == 1, "No edit found");
        assert!(edit.unwrap().choices[0].text.replace("\n", "").eq("What a wonderful day!"));

        
        // Autocorrect Dutch spelling errors using an English instructino prompt?
        let edit = ctx.create_edit(
            crate::edits::EditRequestBuilder::default()
            .model("text-davinci-edit-001")
            .instruction("Correct all spelling mistakes")
            .input("Ik hou van jouw moederr")
            .build()
            .unwrap()
        ).await;

        assert!(edit.is_ok(), "Could not get edit: {}", edit.unwrap_err());
        assert!(edit.as_ref().unwrap().choices.len() == 1, "No edit found");

        // This one might be pushing my luck a bit, but it seems to work
        //assert!(edit.unwrap().choices[0].text.replace("\n", "").eq("Ik hou van jouw moeder"));
    }

    #[tokio::test]
    async fn test_image() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");
        let ctx = ctx.unwrap();

        let image = ctx.create_image(
            crate::image::ImageRequestBuilder::default()
            .prompt("In a realistic style, a ginger cat gracefully walking along a thin brick wall")
            .build()
            .unwrap()
        ).await;

        assert!(image.is_ok(), "Could not get image: {}", image.unwrap_err());
        assert!(image.as_ref().unwrap().data.len() == 1, "No image found");
        assert!(image.as_ref().unwrap().data[0].isURL(), "No image found");
        if let Image::URL(url) = &image.as_ref().unwrap().data[0] {
            println!("{}", url);
        }
    }
}