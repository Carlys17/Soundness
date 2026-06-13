//! Build the SP1 program and generate a Groth16 proof.
//! Writes the proof and public values to ./out/.

use sp1_sdk::{ProverClient, SP1Stdin};

const FIBONACCI_ELF: &[u8] = include_bytes!("../../program/elf/fibonacci-program");

#[tokio::main]
async fn main() {
    let n: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    println!("Building proof for fibonacci({})...", n);

    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // For Soundness Layer testnet we use Groth16 — cheap to verify on chain.
    let (pk, vk) = client.setup(FIBONACCI_ELF);
    let proof = client
        .prove(&pk, stdin)
        .groth16()
        .run()
        .expect("proving failed");

    std::fs::create_dir_all("out").unwrap();
    proof
        .save("out/proof.bin")
        .expect("failed to write proof");
    println!("Proof written to out/proof.bin");

    // The public values are the single fibonacci(n) u64.
    let pv = proof.public_values.as_slice();
    std::fs::write("out/public_values.bin", pv).unwrap();
    println!("Public values written: {} bytes", pv.len());

    // Save verification key for later use.
    std::fs::write("out/vk.bin", bincode::serialize(&vk).unwrap()).unwrap();
    println!("Verification key written to out/vk.bin");
}
