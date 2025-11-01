// tests/infrastructure/embeddings_tests.rs
//
// NOTE: These tests manipulate environment variables and must be run serially
// to avoid interference. Use `cargo test embeddings_tests -- --test-threads=1`
// or rely on the EnvGuard to clean up after each test.
//
use bkmr::domain::embedding::Embedder;
use bkmr::infrastructure::embeddings::OpenAiEmbedding;
use std::env;
use std::sync::Mutex;

// Global mutex to ensure environment variable tests run serially
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Helper to save and restore environment variables
struct EnvGuard {
    vars: Vec<(String, Option<String>)>,
}

impl EnvGuard {
    fn new(vars: &[&str]) -> Self {
        let saved_vars = vars
            .iter()
            .map(|&var| (var.to_string(), env::var(var).ok()))
            .collect();
        
        // Clear all the variables
        for var in vars {
            env::remove_var(var);
        }
        
        Self { vars: saved_vars }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (var, value) in &self.vars {
            env::remove_var(var);
            if let Some(val) = value {
                env::set_var(var, val);
            }
        }
    }
}

#[test]
fn given_no_env_vars_when_create_embedder_then_uses_openai_defaults() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL", "OPENAI_API_KEY"]);
    
    let embedder = OpenAiEmbedding::default();
    
    // Verify defaults via debug output
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("https://api.openai.com"));
    assert!(debug_str.contains("text-embedding-ada-002"));
}

#[test]
fn given_custom_api_url_when_create_embedder_then_uses_custom_url() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL", "OPENAI_API_KEY"]);
    
    env::set_var("OPENAI_API_URL", "http://localhost:11434");
    
    let embedder = OpenAiEmbedding::default();
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("http://localhost:11434"));
}

#[test]
fn given_custom_model_when_create_embedder_then_uses_custom_model() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL", "OPENAI_API_KEY"]);
    
    env::set_var("OPENAI_MODEL", "nomic-embed-text");
    
    let embedder = OpenAiEmbedding::default();
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("nomic-embed-text"));
}

#[test]
fn given_ollama_config_when_create_embedder_then_configures_for_ollama() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL", "OPENAI_API_KEY"]);
    
    // Simulate Ollama configuration
    env::set_var("OPENAI_API_URL", "http://localhost:11434");
    env::set_var("OPENAI_MODEL", "nomic-embed-text");
    env::set_var("OPENAI_API_KEY", "ollama");  // Ollama doesn't need real key
    
    let embedder = OpenAiEmbedding::default();
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("http://localhost:11434"));
    assert!(debug_str.contains("nomic-embed-text"));
}

#[test]
fn given_custom_provider_config_when_create_embedder_then_configures_for_provider() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL", "OPENAI_API_KEY"]);
    
    // Simulate custom provider configuration
    env::set_var("OPENAI_API_URL", "https://api.custom-provider.com");
    env::set_var("OPENAI_MODEL", "custom-embedding-model");
    env::set_var("OPENAI_API_KEY", "sk-custom-key");
    
    let embedder = OpenAiEmbedding::default();
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("https://api.custom-provider.com"));
    assert!(debug_str.contains("custom-embedding-model"));
}

#[test]
fn given_explicit_config_when_create_embedder_then_uses_provided_values() {
    let embedder = OpenAiEmbedding::new(
        "https://custom.endpoint.com".to_string(),
        "custom-model".to_string(),
    );
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("https://custom.endpoint.com"));
    assert!(debug_str.contains("custom-model"));
}

#[test]
fn given_from_env_when_no_vars_set_then_uses_defaults() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL"]);
    
    let embedder = OpenAiEmbedding::from_env();
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("https://api.openai.com"));
    assert!(debug_str.contains("text-embedding-ada-002"));
}

#[test]
fn given_from_env_when_vars_set_then_uses_env_values() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL"]);
    
    env::set_var("OPENAI_API_URL", "http://test.local:8080");
    env::set_var("OPENAI_MODEL", "test-model");
    
    let embedder = OpenAiEmbedding::from_env();
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("http://test.local:8080"));
    assert!(debug_str.contains("test-model"));
}

#[test]
fn given_missing_api_key_when_embed_then_returns_error() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_KEY"]);
    
    let embedder = OpenAiEmbedding::default();
    let result = embedder.embed("test text");
    
    assert!(result.is_err());
    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(err_msg.contains("OPENAI_API_KEY"));
}

#[test]
fn given_backward_compatibility_when_only_api_key_set_then_uses_openai_defaults() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL", "OPENAI_API_KEY"]);
    
    // Only set API key, like in the old behavior
    env::set_var("OPENAI_API_KEY", "sk-test-key");
    
    let embedder = OpenAiEmbedding::default();
    
    // Should still use OpenAI defaults for URL and model
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("https://api.openai.com"));
    assert!(debug_str.contains("text-embedding-ada-002"));
}

#[test]
fn given_partial_config_when_only_url_set_then_uses_default_model() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL"]);
    
    env::set_var("OPENAI_API_URL", "http://localhost:11434");
    
    let embedder = OpenAiEmbedding::from_env();
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("http://localhost:11434"));
    assert!(debug_str.contains("text-embedding-ada-002")); // Default model
}

#[test]
fn given_partial_config_when_only_model_set_then_uses_default_url() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _guard = EnvGuard::new(&["OPENAI_API_URL", "OPENAI_MODEL"]);
    
    env::set_var("OPENAI_MODEL", "custom-model");
    
    let embedder = OpenAiEmbedding::from_env();
    
    let debug_str = format!("{:?}", embedder);
    assert!(debug_str.contains("https://api.openai.com")); // Default URL
    assert!(debug_str.contains("custom-model"));
}

#[cfg(feature = "integration_tests")]
#[test]
fn given_openai_credentials_when_embed_text_then_returns_vector() {
    // This test only runs if OPENAI_API_KEY is actually set
    if env::var("OPENAI_API_KEY").is_err() {
        eprintln!("Skipping integration test: OPENAI_API_KEY not set");
        return;
    }
    
    let embedder = OpenAiEmbedding::default();
    let result = embedder.embed("test text for embedding");
    
    assert!(result.is_ok());
    let embedding = result.unwrap();
    assert!(embedding.is_some());
    
    let vec = embedding.unwrap();
    // OpenAI's text-embedding-ada-002 produces 1536-dimensional vectors
    assert_eq!(vec.len(), 1536);
}
