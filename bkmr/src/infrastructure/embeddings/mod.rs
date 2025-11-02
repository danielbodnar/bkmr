pub(crate) mod dummy_provider;
mod model;
pub mod openai_compatible_provider;
pub mod openai_provider;
pub mod providers;

pub use dummy_provider::DummyEmbedding;
pub use openai_compatible_provider::OpenAiCompatibleEmbedding;
pub use openai_provider::OpenAiEmbedding;
