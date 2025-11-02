use crate::domain::embedding::Embedder;
use crate::domain::error::{DomainError, DomainResult};
use crate::infrastructure::embeddings::model::{EmbeddingRequest, EmbeddingResponse};
use crate::infrastructure::embeddings::providers::{get_provider, ProviderConfig};
use std::any::Any;
use std::env;
use tracing::{debug, instrument};

/// Generic implementation for OpenAI-compatible embedding APIs.
///
/// This provider works with any service that implements the OpenAI embeddings API format,
/// including OpenAI, Voyage AI, Ollama, and other compatible providers.
///
/// # Configuration
///
/// Configuration is done via environment variables:
/// - `OPENAI_API_KEY`: The API key for authentication
/// - `OPENAI_API_BASE`: (Optional) Base URL for the API endpoint
/// - `OPENAI_MODEL`: (Optional) Model name to use for embeddings
/// - `OPENAI_PROVIDER`: (Optional) Named provider (openai, voyageai, ollama, etc.)
///
/// # Limitations
///
/// For providers with non-standard authentication or response formats,
/// this implementation may need to be extended to support provider-specific
/// authentication headers or response parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Creates an embedding provider from environment variables.
    ///
    /// # Environment Variables
    ///
    /// - `OPENAI_API_KEY` (required): API key for authentication
    /// - `OPENAI_PROVIDER` (optional): Named provider (openai, voyageai, ollama, huggingface, local)
    /// - `OPENAI_API_BASE` (optional): Custom API endpoint URL override
    /// - `OPENAI_MODEL` (optional): Model name override
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bkmr::infrastructure::embeddings::OpenAiCompatibleEmbedding;
    ///
    /// std::env::set_var("OPENAI_API_KEY", "sk-...");
    /// std::env::set_var("OPENAI_PROVIDER", "openai");
    ///
    /// let embedder = OpenAiCompatibleEmbedding::from_env();
    /// ```
    ///
    /// # Provider Selection
    ///
    /// If `OPENAI_PROVIDER` matches a known provider, that provider's defaults
    /// are used. Environment variables can override any default setting.
    pub fn from_env() -> Self {
        let provider_name = env::var("OPENAI_PROVIDER")
            .ok()
            .unwrap_or_else(|| "openai".to_string());

        if let Some(config) = get_provider(&provider_name) {
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

    /// Creates a new embedding provider with explicit configuration.
    ///
    /// # Arguments
    ///
    /// * `api_base` - The base URL for the API endpoint (e.g., "https://api.openai.com/v1")
    /// * `model` - The model name to use for embeddings
    ///
    /// # Examples
    ///
    /// ```
    /// use bkmr::infrastructure::embeddings::OpenAiCompatibleEmbedding;
    ///
    /// let embedder = OpenAiCompatibleEmbedding::new(
    ///     "https://api.openai.com/v1".to_string(),
    ///     "text-embedding-ada-002".to_string()
    /// );
    /// ```
    pub fn new(api_base: String, model: String) -> Self {
        Self {
            api_base,
            model,
            provider_config: None,
        }
    }

    /// Creates a provider for a named provider.
    ///
    /// # Arguments
    ///
    /// * `provider_name` - One of: "openai", "voyageai", "ollama", "huggingface", "local"
    ///
    /// # Returns
    ///
    /// `Some(Self)` if the provider is known, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use bkmr::infrastructure::embeddings::OpenAiCompatibleEmbedding;
    ///
    /// let voyageai = OpenAiCompatibleEmbedding::for_provider("voyageai").unwrap();
    /// assert_eq!(voyageai.model, "voyage-3-large");
    /// ```
    pub fn for_provider(provider_name: &str) -> Option<Self> {
        get_provider(provider_name).map(|config| Self {
            api_base: config.api_base.to_string(),
            model: config.default_model.to_string(),
            provider_config: Some(config),
        })
    }

    /// Returns the provider name for error messages and logging.
    fn provider_name(&self) -> &str {
        self.provider_config
            .as_ref()
            .map(|c| c.name)
            .unwrap_or("custom")
    }

    /// Returns authentication header name and value for the configured provider.
    fn get_auth_header(&self, api_key: &str) -> (&'static str, String) {
        if let Some(ref config) = self.provider_config {
            config.get_auth_header(api_key)
        } else {
            ("Authorization", format!("Bearer {}", api_key))
        }
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

        let provider_name = self.provider_name();

        let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
            DomainError::CannotFetchMetadata(format!(
                "OPENAI_API_KEY environment variable not set (provider: {})",
                provider_name
            ))
        })?;

        let client = reqwest::blocking::Client::new();

        let request = EmbeddingRequest {
            input: text,
            model: &self.model,
        };

        let (auth_header_name, auth_header_value) = self.get_auth_header(&api_key);

        let response = client
            .post(format!("{}/embeddings", self.api_base))
            .header(auth_header_name, auth_header_value)
            .json(&request)
            .send()
            .map_err(|e| {
                DomainError::CannotFetchMetadata(format!(
                    "API request failed for provider '{}' (endpoint: {}): {}",
                    provider_name, self.api_base, e
                ))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unable to read error".to_string());

            return Err(DomainError::CannotFetchMetadata(format!(
                "Provider '{}' returned error (status {}): {}",
                provider_name, status, error_text
            )));
        }

        let response_data: EmbeddingResponse = response.json().map_err(|e| {
            DomainError::CannotFetchMetadata(format!(
                "Failed to parse response from provider '{}': {}",
                provider_name, e
            ))
        })?;

        if response_data.data.is_empty() {
            debug!("Provider '{}' returned empty data array", provider_name);
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
    use serial_test::serial;

    #[test]
    #[serial]
    fn given_text_input_when_create_embedding_then_returns_vector() {
        let _ = init_test_env();
        if env::var("OPENAI_API_KEY").is_err() {
            eprintln!("OPENAI_API_KEY environment variable not set");
            return;
        }

        let embedder = OpenAiCompatibleEmbedding::default();
        let result = embedder.embed("test text");
        assert!(result.is_ok());
        let embedding = result.unwrap().unwrap();
        assert!(!embedding.is_empty());
    }

    #[test]
    #[serial]
    fn given_missing_api_key_when_create_embedding_then_returns_error() {
        let api_key_backup = env::var("OPENAI_API_KEY").ok();

        env::remove_var("OPENAI_API_KEY");

        let embedder = OpenAiCompatibleEmbedding::default();
        let result = embedder.embed("test text");
        assert!(result.is_err());

        if let Some(key) = api_key_backup {
            env::set_var("OPENAI_API_KEY", key);
        }
    }

    #[test]
    fn test_provider_selection() {
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
    #[serial]
    fn test_env_override() {
        let saved_provider = env::var("OPENAI_PROVIDER").ok();
        let saved_base = env::var("OPENAI_API_BASE").ok();
        let saved_model = env::var("OPENAI_MODEL").ok();

        env::set_var("OPENAI_PROVIDER", "voyageai");
        env::set_var("OPENAI_API_BASE", "https://custom.example.com/v1");
        env::set_var("OPENAI_MODEL", "custom-model");

        let embedder = OpenAiCompatibleEmbedding::from_env();
        assert_eq!(embedder.api_base, "https://custom.example.com/v1");
        assert_eq!(embedder.model, "custom-model");

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
