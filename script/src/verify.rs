//! Verify a previously generated proof against a known verification key.

use sp1_sdk::ProverClient;

const FIBONACCI_ELF: &[u8] = include_bytes!("../../program/elf/fibonacci-program");

fn main() {
    let proof =
        sp1_sdk::SP1ProofWithPublicValues::load("out/proof.bin").expect("load proof");
    let pv = std::fs::read("out/public_values.bin").expect("read public values");

    let client = ProverClient::new();
    let (_pk, vk) = client.setup(FIBONACCI_ELF);
    client
        .verify(&proof, &vk)
        .expect("verification failed");

    println!("✓ proof is valid");
    println!("  public values ({} bytes): 0x{}", pv.len(), hex::encode(&pv));
}
