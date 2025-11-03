use crate::domain::embedding::Embedder;
use crate::domain::error::{DomainError, DomainResult};
use crate::infrastructure::embeddings::model::{EmbeddingRequest, EmbeddingResponse};
use std::any::Any;
use std::env;
use tracing::{debug, instrument};

/// Implementation using OpenAI's embedding API
#[derive(Debug, Clone)]
pub struct OpenAiEmbedding {
    url: String,
    model: String,
}

impl Default for OpenAiEmbedding {
    fn default() -> Self {
        let url = env::var("OPENAI_API_BASE")
            .unwrap_or_else(|_| "https://api.openai.com".to_string());
        let model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "text-embedding-ada-002".to_string());

        Self { url, model }
    }
}

impl Embedder for OpenAiEmbedding {
    #[instrument]
    fn embed(&self, text: &str) -> DomainResult<Option<Vec<f32>>> {
        debug!("OpenAI embedding request for text length: {}", text.len());

        let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
            DomainError::CannotFetchMetadata(
                "OPENAI_API_KEY environment variable not set".to_string(),
            )
        })?;

        let client = reqwest::blocking::Client::new();

        let request = EmbeddingRequest {
            input: text.to_string(),
            model: self.model.clone(),
        };

        let response = client
            .post(format!("{}/v1/embeddings", self.url))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request)
            .send()
            .map_err(|e| {
                DomainError::CannotFetchMetadata(format!("OpenAI API request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().map_err(|e| {
                DomainError::CannotFetchMetadata(format!("Failed to read error response: {}", e))
            })?;

            return Err(DomainError::CannotFetchMetadata(format!(
                "OpenAI API returned error: {}",
                error_text
            )));
        }

        let response_data: EmbeddingResponse = response.json().map_err(|e| {
            DomainError::CannotFetchMetadata(format!("Failed to parse OpenAI response: {}", e))
        })?;

        if response_data.data.is_empty() {
            debug!("OpenAI API returned empty data array");
            return Ok(None);
        }

        Ok(Some(response_data.data[0].embedding.clone()))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl OpenAiEmbedding {
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
        if env::var("OPENAI_API_KEY").is_err() {
            // exit early if no API key is set
            eprintln!("OpenAI API_KEY environment variable not set");
            return;
        }

        let openai = OpenAiEmbedding::default();
        let result = openai.embed("test text");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().len(), 1536);
    }

    #[test]
    fn given_missing_api_key_when_create_embedding_then_returns_error() {
        // Temporarily unset the API key if it exists
        let key_exists = env::var("OPENAI_API_KEY").is_ok();
        let api_key_backup = if key_exists {
            Some(env::var("OPENAI_API_KEY").unwrap())
        } else {
            None
        };

        env::remove_var("OPENAI_API_KEY");

        let openai = OpenAiEmbedding::default();
        let result = openai.embed("test text");
        assert!(result.is_err());

        // Restore API key if it existed
        if let Some(key) = api_key_backup {
            env::set_var("OPENAI_API_KEY", key);
        }
    }

    #[test]
    fn given_custom_base_url_when_default_then_uses_custom_url() {
        let base_backup = env::var("OPENAI_API_BASE").ok();

        env::set_var("OPENAI_API_BASE", "https://custom.api.com");
        let openai = OpenAiEmbedding::default();

        assert_eq!(openai.url, "https://custom.api.com");

        // Restore or remove
        match base_backup {
            Some(val) => env::set_var("OPENAI_API_BASE", val),
            None => env::remove_var("OPENAI_API_BASE"),
        }
    }

    #[test]
    fn given_custom_model_when_default_then_uses_custom_model() {
        let model_backup = env::var("OPENAI_MODEL").ok();

        env::set_var("OPENAI_MODEL", "text-embedding-3-large");
        let openai = OpenAiEmbedding::default();

        assert_eq!(openai.model, "text-embedding-3-large");

        // Restore or remove
        match model_backup {
            Some(val) => env::set_var("OPENAI_MODEL", val),
            None => env::remove_var("OPENAI_MODEL"),
        }
    }

    #[test]
    fn given_no_custom_config_when_default_then_uses_defaults() {
        let base_backup = env::var("OPENAI_API_BASE").ok();
        let model_backup = env::var("OPENAI_MODEL").ok();

        env::remove_var("OPENAI_API_BASE");
        env::remove_var("OPENAI_MODEL");

        let openai = OpenAiEmbedding::default();

        assert_eq!(openai.url, "https://api.openai.com");
        assert_eq!(openai.model, "text-embedding-ada-002");

        // Restore
        if let Some(val) = base_backup {
            env::set_var("OPENAI_API_BASE", val);
        }
        if let Some(val) = model_backup {
            env::set_var("OPENAI_MODEL", val);
        }
    }

    #[test]
    fn given_new_constructor_when_explicit_values_then_overrides_env() {
        let base_backup = env::var("OPENAI_API_BASE").ok();
        let model_backup = env::var("OPENAI_MODEL").ok();

        env::set_var("OPENAI_API_BASE", "https://env.api.com");
        env::set_var("OPENAI_MODEL", "env-model");

        let openai = OpenAiEmbedding::new(
            "https://explicit.api.com".to_string(),
            "explicit-model".to_string()
        );

        assert_eq!(openai.url, "https://explicit.api.com");
        assert_eq!(openai.model, "explicit-model");

        // Restore
        match base_backup {
            Some(val) => env::set_var("OPENAI_API_BASE", val),
            None => env::remove_var("OPENAI_API_BASE"),
        }
        match model_backup {
            Some(val) => env::set_var("OPENAI_MODEL", val),
            None => env::remove_var("OPENAI_MODEL"),
        }
    }
}
