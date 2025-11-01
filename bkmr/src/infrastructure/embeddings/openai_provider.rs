use crate::domain::embedding::Embedder;
use crate::domain::error::{DomainError, DomainResult};
use crate::infrastructure::embeddings::model::{EmbeddingRequest, EmbeddingResponse};
use std::any::Any;
use std::env;
use tracing::{debug, instrument};

/// Implementation using OpenAI-compatible embedding API
/// 
/// Supports any OpenAI-compatible embeddings provider (OpenAI, Ollama, HuggingFace, Voyage AI, etc.)
/// 
/// ## Configuration
/// 
/// Environment variables (for backward compatibility, all use OPENAI_* prefix):
/// - `OPENAI_API_KEY`: API key for authentication (required for most providers)
/// - `OPENAI_API_BASE`: Base URL for the API endpoint (optional, defaults to "https://api.openai.com/v1")
/// - `OPENAI_MODEL`: Model name to use for embeddings (optional, defaults to "text-embedding-3-small")
/// 
/// ## Supported Providers
/// 
/// This implementation supports any provider that follows the OpenAI embeddings API specification:
/// - **OpenAI**: Use defaults (official OpenAI API)
/// - **Ollama**: Set OPENAI_API_BASE="http://localhost:11434" and OPENAI_MODEL="nomic-embed-text"
/// - **HuggingFace**: Set OPENAI_API_BASE to HF endpoint and appropriate model
/// - **Voyage AI**: Set OPENAI_API_BASE and OPENAI_MODEL accordingly (uses X-Api-Key header)
/// - **Custom**: Any OpenAI-compatible endpoint
/// 
/// ## Authentication
/// 
/// Auth headers are automatically detected based on the URL:
/// - Voyage AI (api.voyageai.com): Uses `X-Api-Key` header
/// - Localhost/Ollama: No authentication required
/// - All others: Uses `Authorization: Bearer` header
/// 
/// ## Non-Compatible Providers
/// 
/// **Note**: This implementation is specifically designed for OpenAI-compatible REST APIs.
/// Providers that use different API patterns (gRPC, different request/response formats,
/// OAuth flows, AWS SigV4, etc.) would require a different architecture with provider-specific
/// adapters. Examples of non-compatible providers:
/// - Cohere (different API format)
/// - Anthropic (different API structure)
/// - Providers requiring OAuth or AWS SigV4
/// - Streaming-only APIs
/// - gRPC-based services
#[derive(Debug, Clone)]
pub struct OpenAiEmbedding {
    url: String,
    model: String,
}

impl Default for OpenAiEmbedding {
    fn default() -> Self {
        Self::from_env()
    }
}

impl OpenAiEmbedding {
    /// Create a new OpenAI-compatible embedder with explicit configuration
    pub fn new(url: String, model: String) -> Self {
        Self { url, model }
    }

    /// Create embedder from environment variables
    /// 
    /// Reads configuration from:
    /// - OPENAI_API_BASE (defaults to "https://api.openai.com/v1")
    /// - OPENAI_MODEL (defaults to "text-embedding-3-small")
    /// 
    /// Note: Also checks legacy OPENAI_API_URL for backward compatibility
    pub fn from_env() -> Self {
        // Check OPENAI_API_BASE first, then fall back to legacy OPENAI_API_URL
        let url = env::var("OPENAI_API_BASE")
            .or_else(|_| env::var("OPENAI_API_URL"))
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        
        let model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "text-embedding-3-small".to_string());
        
        debug!("OpenAI embedder configured with URL: {}, Model: {}", url, model);
        
        Self { url, model }
    }
    
    /// Determine the appropriate authentication header based on the URL
    /// 
    /// - Voyage AI (api.voyageai.com): X-Api-Key
    /// - Localhost/127.0.0.1: No auth
    /// - All others: Authorization: Bearer
    fn get_auth_header(&self, api_key: &str) -> Option<(&'static str, String)> {
        let url_lower = self.url.to_lowercase();
        
        // No auth for localhost/Ollama
        if url_lower.contains("localhost") || url_lower.contains("127.0.0.1") {
            debug!("No authentication required for localhost");
            return None;
        }
        
        // Voyage AI uses X-Api-Key
        if url_lower.contains("api.voyageai.com") {
            debug!("Using X-Api-Key authentication for Voyage AI");
            return Some(("X-Api-Key", api_key.to_string()));
        }
        
        // Default: Bearer token
        debug!("Using Bearer token authentication");
        Some(("Authorization", format!("Bearer {}", api_key)))
    }
}

impl Embedder for OpenAiEmbedding {
    #[instrument]
    fn embed(&self, text: &str) -> DomainResult<Option<Vec<f32>>> {
        debug!("OpenAI embedding request for text length: {}", text.len());

        let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
        
        // Validate API key if authentication is required
        if let Some(_) = self.get_auth_header(&api_key) {
            if api_key.is_empty() {
                return Err(DomainError::CannotFetchMetadata(
                    "OPENAI_API_KEY environment variable not set".to_string(),
                ));
            }
        }

        let client = reqwest::blocking::Client::new();

        let request = EmbeddingRequest {
            input: text.to_string(),
            model: self.model.clone(),
        };

        // Build request with appropriate auth header
        let mut request_builder = client
            .post(format!("{}/embeddings", self.url))
            .json(&request);
        
        // Add auth header if required
        if let Some((header_name, header_value)) = self.get_auth_header(&api_key) {
            request_builder = request_builder.header(header_name, header_value);
        }
        
        let response = request_builder.send()
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
}
