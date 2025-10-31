use crate::domain::embedding::Embedder;
use crate::domain::error::{DomainError, DomainResult};
use crate::infrastructure::embeddings::model::{EmbeddingRequest, EmbeddingResponse};
use std::any::Any;
use std::env;
use tracing::{debug, instrument};

/// Implementation using Voyage AI's embedding API
#[derive(Debug, Clone)]
pub struct VoyageAiEmbedding {
    url: String,
    model: String,
}

impl Default for VoyageAiEmbedding {
    fn default() -> Self {
        Self {
            url: "https://api.voyageai.com".to_string(),
            model: "voyage-3-large".to_string(),
        }
    }
}

impl Embedder for VoyageAiEmbedding {
    #[instrument]
    fn embed(&self, text: &str) -> DomainResult<Option<Vec<f32>>> {
        debug!("Voyage AI embedding request for text length: {}", text.len());

        let api_key = env::var("VOYAGE_API_KEY").map_err(|_| {
            DomainError::CannotFetchMetadata(
                "VOYAGE_API_KEY environment variable not set".to_string(),
            )
        })?;

        let client = reqwest::blocking::Client::new();

        let request = EmbeddingRequest {
            input: text.to_string(),
            model: self.model.clone(),
        };

        let response = client
            .post(format!("{}/v1/embeddings", self.url))
            .header("X-Api-Key", api_key)
            .json(&request)
            .send()
            .map_err(|e| {
                DomainError::CannotFetchMetadata(format!("Voyage AI API request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().map_err(|e| {
                DomainError::CannotFetchMetadata(format!("Failed to read error response: {}", e))
            })?;

            return Err(DomainError::CannotFetchMetadata(format!(
                "Voyage AI API returned error: {}",
                error_text
            )));
        }

        let response_data: EmbeddingResponse = response.json().map_err(|e| {
            DomainError::CannotFetchMetadata(format!("Failed to parse Voyage AI response: {}", e))
        })?;

        if response_data.data.is_empty() {
            debug!("Voyage AI API returned empty data array");
            return Ok(None);
        }

        Ok(Some(response_data.data[0].embedding.clone()))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl VoyageAiEmbedding {
    pub fn new(url: String, model: String) -> Self {
        Self { url, model }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::testing::init_test_env;

    #[test]
    fn given_text_input_when_create_embedding_then_returns_vector() {
        let _ = init_test_env();
        if env::var("VOYAGE_API_KEY").is_err() {
            // exit early if no API key is set
            eprintln!("VOYAGE_API_KEY environment variable not set");
            return;
        }

        let voyageai = VoyageAiEmbedding::default();
        let result = voyageai.embed("test text");
        assert!(result.is_ok());
        // voyage-3-large produces 2048-dimensional embeddings
        assert_eq!(result.unwrap().unwrap().len(), 2048);
    }

    #[test]
    fn given_missing_api_key_when_create_embedding_then_returns_error() {
        // Temporarily unset the API key if it exists
        let api_key_backup = env::var("VOYAGE_API_KEY").ok();

        env::remove_var("VOYAGE_API_KEY");

        let voyageai = VoyageAiEmbedding::default();
        let result = voyageai.embed("test text");
        assert!(result.is_err());

        // Restore API key if it existed
        if let Some(key) = api_key_backup {
            env::set_var("VOYAGE_API_KEY", key);
        }
    }
}
