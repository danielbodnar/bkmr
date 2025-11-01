/// Configuration for common OpenAI-compatible embedding providers
/// 
/// This module defines known embedding providers that are compatible with the OpenAI embeddings API.
/// Each provider has a default API endpoint and model.
/// 
/// Note: Some providers may require different authentication headers or response formats.
/// Currently, this implementation assumes full OpenAI API compatibility.
/// Future versions may need to abstract authentication and response parsing to support
/// providers with different authentication schemes (e.g., API-Key vs Bearer tokens).

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub name: &'static str,
    pub api_base: &'static str,
    pub default_model: &'static str,
    pub auth_header: AuthHeaderType,
}

#[derive(Debug, Clone)]
pub enum AuthHeaderType {
    /// Authorization: Bearer {token}
    Bearer,
    /// X-Api-Key: {token}
    ApiKey,
}

impl ProviderConfig {
    pub fn get_auth_header(&self, api_key: &str) -> (&'static str, String) {
        match self.auth_header {
            AuthHeaderType::Bearer => ("Authorization", format!("Bearer {}", api_key)),
            AuthHeaderType::ApiKey => ("X-Api-Key", api_key.to_string()),
        }
    }
}

/// Known OpenAI-compatible embedding providers with their default configurations
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
            auth_header: AuthHeaderType::Bearer, // Ollama typically doesn't require auth
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

    // Generic local provider (useful for testing or custom OpenAI-compatible endpoints)
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

/// Get provider configuration by name
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
