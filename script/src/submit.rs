//! Submit a generated proof to the Soundness Layer testnet.

use serde::Serialize;

const TESTNET_RPC: &str = "https://testnet-rpc.soundness.xyz";

#[derive(Serialize)]
struct SubmitReq {
    proof_b64: String,
    public_values_b64: String,
    vk_b64: String,
    program: String, // e.g. "fibonacci"
    metadata: serde_json::Value,
}

#[derive(Serialize)]
struct SubmitResp {
    proof_id: String,
    status: String,
    explorer_url: String,
}

#[tokio::main]
async fn main() {
    let proof = std::fs::read("out/proof.bin").expect("proof");
    let pv = std::fs::read("out/public_values.bin").expect("public values");
    let vk = std::fs::read("out/vk.bin").expect("vk");

    let body = SubmitReq {
        proof_b64: base64_encode(&proof),
        public_values_b64: base64_encode(&pv),
        vk_b64: base64_encode(&vk),
        program: "fibonacci".to_string(),
        metadata: serde_json::json!({
            "source": "https://github.com/Carlys17/Soundness",
            "version": "0.1.0",
        }),
    };

    println!("Submitting to {}...", TESTNET_RPC);
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/v1/proofs/submit", TESTNET_RPC))
        .json(&body)
        .send()
        .await
        .expect("send");

    if !resp.status().is_success() {
        eprintln!("submit failed: {} {}", resp.status(), resp.text().await.unwrap_or_default());
        std::process::exit(1);
    }

    let out: SubmitResp = resp.json().await.expect("parse response");
    println!("✓ proof id: {}", out.proof_id);
    println!("  status: {}", out.status);
    println!("  explorer: {}", out.explorer_url);
}

fn base64_encode(b: &[u8]) -> String {
    use std::io::Write;
    let mut out = Vec::new();
    let mut enc = base64::write::EncoderWriter::new(&mut out, &base64::engine::general_purpose::STANDARD);
    enc.write_all(b).unwrap();
    drop(enc);
    String::from_utf8(out).unwrap()
}
