use crate::domain::embedding::Embedder;
use crate::domain::error::{DomainError, DomainResult};
use crate::infrastructure::embeddings::model::{EmbeddingRequest, EmbeddingResponse};
use crate::infrastructure::embeddings::providers::{get_provider, ProviderConfig};
use std::any::Any;
use std::env;
use tracing::{debug, instrument};

/// Generic implementation for OpenAI-compatible embedding APIs
/// 
/// This provider works with any service that implements the OpenAI embeddings API format,
/// including OpenAI, Voyage AI, Ollama, and other compatible providers.
/// 
/// Configuration is done via environment variables:
/// - OPENAI_API_KEY: The API key for authentication
/// - OPENAI_API_BASE: (Optional) Base URL for the API endpoint
/// - OPENAI_MODEL: (Optional) Model name to use for embeddings
/// - OPENAI_PROVIDER: (Optional) Named provider (openai, voyageai, ollama, etc.)
/// 
/// Note: For providers with non-standard authentication or response formats,
/// this implementation may need to be extended to support provider-specific
/// authentication headers or response parsing.
#[derive(Debug, Clone)]
pub struct OpenAiCompatibleEmbedding {
    api_base: String,
    model: String,
    provider_config: Option<ProviderConfig>,
}

impl Default for OpenAiCompatibleEmbedding {
    fn default() -> Self {
        Self::from_env()
    }
}

impl OpenAiCompatibleEmbedding {
    /// Create a new embedding provider from environment variables
    pub fn from_env() -> Self {
        // Check for provider name first
        let provider_name = env::var("OPENAI_PROVIDER")
            .ok()
            .unwrap_or_else(|| "openai".to_string());

        if let Some(config) = get_provider(&provider_name) {
            // Use provider config as base, but allow overrides
            let api_base = env::var("OPENAI_API_BASE")
                .ok()
                .unwrap_or_else(|| config.api_base.to_string());
            let model = env::var("OPENAI_MODEL")
                .ok()
                .unwrap_or_else(|| config.default_model.to_string());

            Self {
                api_base,
                model,
                provider_config: Some(config),
            }
        } else {
            // Fallback to manual configuration
            let api_base = env::var("OPENAI_API_BASE")
                .ok()
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
            let model = env::var("OPENAI_MODEL")
                .ok()
                .unwrap_or_else(|| "text-embedding-ada-002".to_string());

            Self {
                api_base,
                model,
                provider_config: None,
            }
        }
    }

    /// Create a new embedding provider with explicit configuration
    pub fn new(api_base: String, model: String) -> Self {
        Self {
            api_base,
            model,
            provider_config: None,
        }
    }

    /// Create a new embedding provider for a named provider
    pub fn for_provider(provider_name: &str) -> Option<Self> {
        get_provider(provider_name).map(|config| Self {
            api_base: config.api_base.to_string(),
            model: config.default_model.to_string(),
            provider_config: Some(config),
        })
    }
}

impl Embedder for OpenAiCompatibleEmbedding {
    #[instrument]
    fn embed(&self, text: &str) -> DomainResult<Option<Vec<f32>>> {
        debug!(
            "OpenAI-compatible embedding request for text length: {}, api_base: {}, model: {}",
            text.len(),
            self.api_base,
            self.model
        );

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

        // Determine authentication header based on provider config
        let (auth_header_name, auth_header_value) = if let Some(ref config) = self.provider_config
        {
            config.get_auth_header(&api_key)
        } else {
            // Default to Bearer token (OpenAI style)
            ("Authorization", format!("Bearer {}", api_key))
        };

        let response = client
            .post(format!("{}/embeddings", self.api_base))
            .header(auth_header_name, auth_header_value)
            .json(&request)
            .send()
            .map_err(|e| {
                DomainError::CannotFetchMetadata(format!("API request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().map_err(|e| {
                DomainError::CannotFetchMetadata(format!("Failed to read error response: {}", e))
            })?;

            return Err(DomainError::CannotFetchMetadata(format!(
                "API returned error (status {}): {}",
                status, error_text
            )));
        }

        let response_data: EmbeddingResponse = response.json().map_err(|e| {
            DomainError::CannotFetchMetadata(format!("Failed to parse API response: {}", e))
        })?;

        if response_data.data.is_empty() {
            debug!("API returned empty data array");
            return Ok(None);
        }

        Ok(Some(response_data.data[0].embedding.clone()))
    }

    fn as_any(&self) -> &dyn Any {
        self
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
            eprintln!("OPENAI_API_KEY environment variable not set");
            return;
        }

        let embedder = OpenAiCompatibleEmbedding::default();
        let result = embedder.embed("test text");
        assert!(result.is_ok());
        // OpenAI's text-embedding-ada-002 produces 1536-dimensional embeddings
        let embedding = result.unwrap().unwrap();
        assert!(!embedding.is_empty());
    }

    #[test]
    fn given_missing_api_key_when_create_embedding_then_returns_error() {
        // Temporarily unset the API key if it exists
        let api_key_backup = env::var("OPENAI_API_KEY").ok();

        env::remove_var("OPENAI_API_KEY");

        let embedder = OpenAiCompatibleEmbedding::default();
        let result = embedder.embed("test text");
        assert!(result.is_err());

        // Restore API key if it existed
        if let Some(key) = api_key_backup {
            env::set_var("OPENAI_API_KEY", key);
        }
    }

    #[test]
    fn test_provider_selection() {
        // Test that provider can be selected
        let voyageai = OpenAiCompatibleEmbedding::for_provider("voyageai");
        assert!(voyageai.is_some());
        let voyageai = voyageai.unwrap();
        assert_eq!(voyageai.model, "voyage-3-large");
        assert!(voyageai.api_base.contains("voyageai.com"));

        let openai = OpenAiCompatibleEmbedding::for_provider("openai");
        assert!(openai.is_some());
        let openai = openai.unwrap();
        assert_eq!(openai.model, "text-embedding-ada-002");
        assert!(openai.api_base.contains("api.openai.com"));
    }

    #[test]
    fn test_env_override() {
        // Save current env
        let saved_provider = env::var("OPENAI_PROVIDER").ok();
        let saved_base = env::var("OPENAI_API_BASE").ok();
        let saved_model = env::var("OPENAI_MODEL").ok();

        // Set test values
        env::set_var("OPENAI_PROVIDER", "voyageai");
        env::set_var("OPENAI_API_BASE", "https://custom.example.com/v1");
        env::set_var("OPENAI_MODEL", "custom-model");

        let embedder = OpenAiCompatibleEmbedding::from_env();
        assert_eq!(embedder.api_base, "https://custom.example.com/v1");
        assert_eq!(embedder.model, "custom-model");

        // Restore env
        env::remove_var("OPENAI_PROVIDER");
        env::remove_var("OPENAI_API_BASE");
        env::remove_var("OPENAI_MODEL");
        if let Some(v) = saved_provider {
            env::set_var("OPENAI_PROVIDER", v);
        }
        if let Some(v) = saved_base {
            env::set_var("OPENAI_API_BASE", v);
        }
        if let Some(v) = saved_model {
            env::set_var("OPENAI_MODEL", v);
        }
    }
}
