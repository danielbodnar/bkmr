//! Provider configuration for OpenAI-compatible embedding APIs.
//!
//! This module maintains a registry of known embedding providers with their
//! default endpoints, models, and authentication schemes.
//!
//! # Adding New Providers
//!
//! To add a provider, insert it into the HashMap in [`get_known_providers`]:
//!
//! ```
//! # use std::collections::HashMap;
//! # use bkmr::infrastructure::embeddings::providers::{ProviderConfig, AuthHeaderType};
//! let mut providers = HashMap::new();
//! providers.insert("my-provider", ProviderConfig {
//!     name: "my-provider",
//!     api_base: "https://api.example.com/v1",
//!     default_model: "my-model",
//!     auth_header: AuthHeaderType::Bearer,
//! });
//! ```
//!
//! # Limitations
//!
//! All providers must be OpenAI API-compatible. Providers with significantly
//! different request/response formats require additional abstraction.

use std::collections::HashMap;

/// Configuration for a specific embedding provider.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProviderConfig {
    pub name: &'static str,
    pub api_base: &'static str,
    pub default_model: &'static str,
    pub auth_header: AuthHeaderType,
}

/// Authentication header type for API requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthHeaderType {
    /// Authorization: Bearer {token}
    Bearer,
    /// X-Api-Key: {token}
    ApiKey,
}

impl ProviderConfig {
    /// Returns the authentication header name and value for this provider.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for authentication
    ///
    /// # Returns
    ///
    /// A tuple of (header_name, header_value)
    pub fn get_auth_header(&self, api_key: &str) -> (&'static str, String) {
        match self.auth_header {
            AuthHeaderType::Bearer => ("Authorization", format!("Bearer {}", api_key)),
            AuthHeaderType::ApiKey => ("X-Api-Key", api_key.to_string()),
        }
    }
}

/// Returns a map of known OpenAI-compatible embedding providers.
///
/// Supported providers:
/// - `openai`: OpenAI's official API
/// - `voyageai`: Voyage AI embeddings
/// - `ollama`: Local Ollama instance
/// - `huggingface`: HuggingFace inference API
/// - `local`: Generic local endpoint for testing
pub fn get_known_providers() -> HashMap<&'static str, ProviderConfig> {
    let mut providers = HashMap::new();

    providers.insert(
        "openai",
        ProviderConfig {
            name: "openai",
            api_base: "https://api.openai.com/v1",
            default_model: "text-embedding-ada-002",
            auth_header: AuthHeaderType::Bearer,
        },
    );

    providers.insert(
        "voyageai",
        ProviderConfig {
            name: "voyageai",
            api_base: "https://api.voyageai.com/v1",
            default_model: "voyage-3-large",
            auth_header: AuthHeaderType::ApiKey,
        },
    );

    providers.insert(
        "ollama",
        ProviderConfig {
            name: "ollama",
            api_base: "http://localhost:11434/v1",
            default_model: "nomic-embed-text",
            auth_header: AuthHeaderType::Bearer,
        },
    );

    providers.insert(
        "huggingface",
        ProviderConfig {
            name: "huggingface",
            api_base: "https://api-inference.huggingface.co/models",
            default_model: "sentence-transformers/all-MiniLM-L6-v2",
            auth_header: AuthHeaderType::Bearer,
        },
    );

    providers.insert(
        "local",
        ProviderConfig {
            name: "local",
            api_base: "http://localhost:8080/v1",
            default_model: "default",
            auth_header: AuthHeaderType::Bearer,
        },
    );

    providers
}

/// Retrieves provider configuration by name.
///
/// # Arguments
///
/// * `name` - The provider name (e.g., "openai", "voyageai")
///
/// # Returns
///
/// `Some(ProviderConfig)` if the provider is known, `None` otherwise.
///
/// # Examples
///
/// ```
/// use bkmr::infrastructure::embeddings::providers::get_provider;
///
/// let openai = get_provider("openai").unwrap();
/// assert_eq!(openai.name, "openai");
/// ```
pub fn get_provider(name: &str) -> Option<ProviderConfig> {
    get_known_providers().get(name).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_providers_exist() {
        let providers = get_known_providers();
        assert!(providers.contains_key("openai"));
        assert!(providers.contains_key("voyageai"));
        assert!(providers.contains_key("ollama"));
        assert!(providers.contains_key("huggingface"));
    }

    #[test]
    fn test_get_provider() {
        let openai = get_provider("openai").unwrap();
        assert_eq!(openai.name, "openai");
        assert_eq!(openai.default_model, "text-embedding-ada-002");
    }

    #[test]
    fn test_bearer_auth_header() {
        let config = ProviderConfig {
            name: "test",
            api_base: "http://test",
            default_model: "test-model",
            auth_header: AuthHeaderType::Bearer,
        };
        let (header_name, header_value) = config.get_auth_header("test_key");
        assert_eq!(header_name, "Authorization");
        assert_eq!(header_value, "Bearer test_key");
    }

    #[test]
    fn test_apikey_auth_header() {
        let config = ProviderConfig {
            name: "test",
            api_base: "http://test",
            default_model: "test-model",
            auth_header: AuthHeaderType::ApiKey,
        };
        let (header_name, header_value) = config.get_auth_header("test_key");
        assert_eq!(header_name, "X-Api-Key");
        assert_eq!(header_value, "test_key");
    }
}
