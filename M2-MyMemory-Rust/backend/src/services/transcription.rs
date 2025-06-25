use axum::{Json};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Deserialize)]
pub struct TranscribeRequest {
    pub file_path: String,
}

#[derive(Serialize)]
pub struct TranscribeResponse {
    pub transcript: String,
}

pub async fn transcribe(Json(payload): Json<TranscribeRequest>) -> Json<TranscribeResponse> {
    let output = Command::new("bash")
        .arg("run_transcription.sh")
        .arg(&payload.file_path)
        .output()
        .expect("Failed to run whisper.cpp");

    let transcript = String::from_utf8_lossy(&output.stdout).to_string();
    Json(TranscribeResponse { transcript })
}
