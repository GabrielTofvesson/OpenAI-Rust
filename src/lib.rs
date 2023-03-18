pub mod context;
pub mod model;
pub mod completion;
pub mod chat;
pub mod edits;
pub mod image;
pub mod image_edit;
pub mod image_variation;
pub mod embedding;
pub mod transcription;
pub mod translation;
pub mod file;
pub mod fine_tune;

pub mod util;

#[cfg(test)]
mod tests {
    use tokio::fs::File;

    use crate::chat::{ChatHistoryBuilder, ChatMessage, Role};
    use crate::context::Context;
    use crate::completion::CompletionRequestBuilder;
    use crate::image::{Image, ResponseFormat, ImageRequestBuilder};
    use crate::edits::EditRequestBuilder;
    use crate::image_edit::ImageEditRequestBuilder;
    use crate::image_variation::ImageVariationRequestBuilder;
    use crate::embedding::EmbeddingRequestBuilder;
    use crate::transcription::{TranscriptionRequestBuilder, AudioFile};
    use crate::translation::TranslationRequestBuilder;

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
            ChatHistoryBuilder::default()
                .messages(vec![ChatMessage::new(Role::User, "Respond to this message with 'this is a test'")])
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
            EditRequestBuilder::default()
                .model("text-davinci-edit-001")
                .instruction("Correct all spelling mistakes")
                .input("What a wnoderful day!")
                .build()
                .unwrap()
        ).await;

        assert!(edit.is_ok(), "Could not get edit: {}", edit.unwrap_err());
        assert!(edit.as_ref().unwrap().choices.len() == 1, "No edit found");
        assert!(edit.unwrap().choices[0].text.replace("\n", "").eq("What a wonderful day!"));
    }

    #[tokio::test]
    async fn test_image() {
        const IMAGE_PROMPT: &str = "A real ginger cat gracefully walking along a real, thin brick wall";
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");
        let ctx = ctx.unwrap();

        let image = ctx.create_image(
            ImageRequestBuilder::default()
                .prompt(IMAGE_PROMPT)
                .response_format(ResponseFormat::URL)
                .build()
                .unwrap()
        ).await;

        assert!(image.is_ok(), "Could not get image: {}", image.unwrap_err());
        assert!(image.as_ref().unwrap().data.len() == 1, "No image found");
        assert!(matches!(image.as_ref().unwrap().data[0], Image::URL(_)), "No image found");
        println!("Image prompt: {IMAGE_PROMPT}");
        match image.unwrap().data[0] {
            Image::URL(ref url) => {
                println!("Generated test image URL: {url}");
            }
            Image::Base64(ref b64) => {
                println!("Generated test image Base64: {b64}");
            }
        }
    }

    #[tokio::test]
    async fn test_image_edit() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");
        let ctx = ctx.unwrap();

        let image = ctx.create_image_edit(
            ImageEditRequestBuilder::default()
                .image(File::open("clown.png").await.unwrap())
                .prompt("Clown with a round, blue nose")
                .build()
                .unwrap()
        ).await;

        assert!(image.is_ok(), "Could not get image: {}", image.unwrap_err());
        assert!(image.as_ref().unwrap().data.len() == 1, "No image found");
        assert!(matches!(image.as_ref().unwrap().data[0], Image::URL(_)), "No image found");
        match image.unwrap().data[0] {
            Image::URL(ref url) => {
                println!("Generated edited image URL: {url}");
            }
            Image::Base64(ref b64) => {
                println!("Generated edited image Base64: {b64}");
            }
        }
    }

    #[tokio::test]
    async fn test_image_variation() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");
        let ctx = ctx.unwrap();

        let image = ctx.create_image_variation(
            ImageVariationRequestBuilder::default()
                .image(File::open("clown_original.png").await.unwrap())
                .build()
                .unwrap()
        ).await;

        assert!(image.is_ok(), "Could not get image: {}", image.unwrap_err());
        assert!(image.as_ref().unwrap().data.len() == 1, "No image found");
        assert!(matches!(image.as_ref().unwrap().data[0], Image::URL(_)), "No image found");
        match image.unwrap().data[0] {
            Image::URL(ref url) => {
                println!("Generated image variation URL: {url}");
            }
            Image::Base64(ref b64) => {
                println!("Generated image variation Base64: {b64}");
            }
        }
    }

    #[tokio::test]
    async fn test_embedding() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");
        let ctx = ctx.unwrap();

        let embeddings = ctx.create_embedding(
            EmbeddingRequestBuilder::default()
                .model("text-embedding-ada-002")
                .input("word sentence paragraph lorem ipsum dolor sit amet")
                .build()
                .unwrap()
        ).await;

        assert!(embeddings.is_ok(), "Could not get embeddings: {}", embeddings.unwrap_err());
        assert!(embeddings.as_ref().unwrap().data.len() == 1, "No embeddings found");
        assert!(embeddings.as_ref().unwrap().data[0].embedding.len() > 0, "No embeddings found");
        println!("Embeddings: {:?}", embeddings.unwrap().data[0].embedding);
    }

    #[tokio::test]
    async fn test_transcription() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");
        let ctx = ctx.unwrap();

        // Original script: "Hello. This is a sample piece of audio for which the whisper AI will generate a transcript"
        // Expected result: "Hello, this is a sample piece of audio for which the Whisper AI will generate a transcript."
        let transcription = ctx.create_transcription(
            TranscriptionRequestBuilder::default()
                .model("whisper-1")
                .file(AudioFile::MP3(File::open("sample_audio.mp3").await.unwrap()))
                .build()
                .unwrap()
        ).await;

        assert!(transcription.is_ok(), "Could not get transcription: {}", transcription.unwrap_err());
        println!("Transcription: {:?}", transcription.unwrap().text);
    }

    #[tokio::test]
    async fn test_translation() {
        let ctx = get_api();
        assert!(ctx.is_ok(), "Could not load context");
        let ctx = ctx.unwrap();

        let translation = ctx.create_translation(
            TranslationRequestBuilder::default()
                .model("whisper-1")
                .prompt("[English]") // Without this, Whisper just responds with the French transcript?
                .file(AudioFile::MP3(File::open("french.mp3").await.unwrap()))
                .build()
                .unwrap()
        ).await;

        assert!(translation.is_ok(), "Could not get translation: {}", translation.unwrap_err());
        println!("Translation: {:?}", translation.unwrap().text);
    }
}