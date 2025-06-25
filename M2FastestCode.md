Below is the full **project folder structure and code layout** for your high-performance **M2 (My Memory)** app using:

* `whisper.cpp` – transcription (C++)
* `llama.cpp` – summarization (C++)
* `Rust` – web API to glue it all together

---

## 📁 Project Structure

```
m2/
├── api/                         # Rust API server
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── whisper.cpp/                 # C++ transcription engine
│   ├── main                     # Compiled binary (after build)
│   └── models/
│       └── ggml-base.en.bin     # Whisper model
├── llama.cpp/                   # C++ summarizer engine
│   ├── build/
│   │   └── main                 # Compiled binary
│   └── models/
│       └── ggml-model-q4_0.gguf # Quantized LLM model
├── samples/
│   └── jfk.wav                  # Sample audio file
└── README.md
```

---

## 📦 `api/Cargo.toml`

```toml
[package]
name = "m2-api"
version = "0.1.0"
edition = "2021"

[dependencies]
warp = "0.3"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

---

## 🦀 `api/src/main.rs`

```rust
use warp::{Filter, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Deserialize)]
struct TranscribeReq {
    audio_path: String,
}

#[derive(Serialize)]
struct TranscribeRes {
    transcript: String,
}

async fn transcribe_handler(req: TranscribeReq) -> Result<impl warp::Reply, warp::Rejection> {
    let output = Command::new("../whisper.cpp/main")
        .args(&["-m", "../whisper.cpp/models/ggml-base.en.bin", "-f", &req.audio_path, "-t", "4"])
        .output()
        .expect("failed to run whisper");

    let transcript = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(warp::reply::json(&TranscribeRes { transcript }))
}

#[derive(Deserialize)]
struct SummarizeReq {
    text: String,
}

#[derive(Serialize)]
struct SummarizeRes {
    summary: String,
}

async fn summarize_handler(req: SummarizeReq) -> Result<impl warp::Reply, warp::Rejection> {
    let prompt = format!("Summarize the following:\n{}", req.text);
    let output = Command::new("../llama.cpp/build/main")
        .args(&["-m", "../llama.cpp/models/ggml-model-q4_0.gguf", "-p", &prompt, "-t", "4"])
        .output()
        .expect("failed to run llama");

    let summary = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(warp::reply::json(&SummarizeRes { summary }))
}

#[tokio::main]
async fn main() {
    let transcribe_route = warp::post()
        .and(warp::path("transcribe"))
        .and(warp::body::json())
        .and_then(transcribe_handler);

    let summarize_route = warp::post()
        .and(warp::path("summarize"))
        .and(warp::body::json())
        .and_then(summarize_handler);

    let routes = transcribe_route.or(summarize_route);
    println!("Server running at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
```

---

## 🔧 Build Steps

### 1. **Build whisper.cpp**

```bash
cd whisper.cpp
make -j
bash models/download-ggml-model.sh base.en
```

### 2. **Build llama.cpp**

```bash
cd llama.cpp
mkdir build && cd build
cmake .. -DLLAMA_CUBLAS=ON    # or just `cmake ..` if no CUDA
cmake --build . --config Release
```

Download a model (e.g. `llama-2-7b.Q4_K_M.gguf` from Hugging Face).

---

### 3. **Run the Rust API**

```bash
cd api
cargo run
```

---

## ✅ Test the API

### Transcription

```bash
curl -X POST http://localhost:3030/transcribe \
  -H "Content-Type: application/json" \
  -d '{"audio_path": "../samples/jfk.wav"}'
```

### Summarization

```bash
curl -X POST http://localhost:3030/summarize \
  -H "Content-Type: application/json" \
  -d '{"text": "John F. Kennedy delivered his inaugural address..."}'
```

---

## 📘 README.md (optional)

```markdown
# M2 – My Memory

A blazing-fast local assistant for recording, transcribing, and summarizing conversations using:
- `whisper.cpp` for speech-to-text
- `llama.cpp` for LLM-based summarization
- `Rust + Warp` for fast HTTP API glue

## Requirements
- Rust (stable)
- CMake + make
- A CUDA GPU (optional, but speeds things up)

## Usage
1. `cargo run` in `/api`
2. Use `/transcribe` and `/summarize` POST endpoints
```

---


