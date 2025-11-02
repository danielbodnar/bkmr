use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct EmbeddingRequest<'a> {
    pub(crate) input: &'a str,
    pub(crate) model: &'a str,
}

#[derive(Deserialize)]
pub struct EmbeddingResponse {
    pub(crate) data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
pub struct EmbeddingData {
    pub(crate) embedding: Vec<f32>,
}
