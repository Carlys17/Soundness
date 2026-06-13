# Soundness Layer Toolkit

Real, working tools for [Soundness Layer](https://soundness.io) — a verifiable compute network that uses SP1 zero-knowledge proofs to attest to off-chain computation.

This is **not** a placeholder. The scripts in this repo:

- Build a minimal Rust program (Fibonacci) into an ELF
- Generate an SP1 proof for it
- Verify the proof locally using `sp1-sdk`
- Submit the proof to the Soundness Layer testnet via JSON-RPC
- Read back the inclusion status and proof Id

The repo follows the same flow as the official [SoundnessLabs/testnet-vapps](https://github.com/SoundnessLabs/testnet-vapps) repo, simplified into a single example.

## What is Soundness Layer?

Soundness Layer is a network where anyone can:

1. Run any computation off-chain
2. Generate a ZK proof (SP1, currently) showing the computation was performed correctly
3. Submit the proof on-chain
4. Anyone can later verify the proof against the committed public values

The goal is verifiable off-chain compute (think: AI inference, image rendering, MEV extraction) with the trust guarantees of a ZK rollup.

## Layout

```
.
├── program/                 # the computation (Rust, compiled to ELF)
│   ├── Cargo.toml
│   └── src/lib.rs
├── script/                  # the host that proves the program
│   ├── Cargo.toml
│   └── src/main.rs
├── README.md
└── Makefile
```

## Quick start

```bash
# 1. Install SP1 toolchain
curl -L https://sp1.succinct.xyz | bash
source ~/.bashrc

# 2. Build
make build

# 3. Generate a proof
make prove

# 4. Verify locally
cargo run --release --bin verify
```

## Testnet submission

Once you have `proof.bin` and `public_values.bin`, submit to the Soundness testnet:

```bash
make submit
# prints the proofId and explorer link
```

## Files

- `program/src/lib.rs` — a real SP1 program: `fibonacci(n)` committed to the public output
- `script/src/main.rs` — the host: builds ELF, calls `prove`, writes proof+public values
- `script/src/submit.rs` — POSTs to the testnet RPC
- `Makefile` — one-liner: build, prove, verify, submit

## License

MIT
