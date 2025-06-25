use axum::{Router, routing::post};
use crate::services::{transcription::transcribe, summarization::summarize, search::vector_search};

pub fn create_routes() -> Router {
    Router::new()
        .route("/transcribe", post(transcribe))
        .route("/summarize", post(summarize))
        .route("/search", post(vector_search))
}
