use axum::{Json};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Deserialize)]
pub struct SummarizeRequest {
    pub text: String,
}

#[derive(Serialize)]
pub struct SummarizeResponse {
    pub summary: String,
}

pub async fn summarize(Json(payload): Json<SummarizeRequest>) -> Json<SummarizeResponse> {
    let output = Command::new("bash")
        .arg("run_summarizer.sh")
        .arg(&payload.text)
        .output()
        .expect("Failed to run summarizer");

    let summary = String::from_utf8_lossy(&output.stdout).to_string();
    Json(SummarizeResponse { summary })
}
