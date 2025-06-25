use axum::{Json};
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<String>,
}

pub async fn vector_search(Json(payload): Json<SearchRequest>) -> Json<SearchResponse> {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:6333/collections/memory/points/search")
        .json(&serde_json::json!({"vector": payload.query, "top": 5}))
        .send()
        .await.unwrap()
        .text().await.unwrap();

    Json(SearchResponse { results: vec![res] })
}
