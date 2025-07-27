```
# Solochain Extrinsics Signer – sr25519 + Dilithium

Rust-based project to sign and build extrinsics for a local Solochain node.  
Supports traditional signatures (sr25519 via `subxt`) and post-quantum signatures (Dilithium via `pqcrypto`).

─────────────────────────────────────────────────────
📁 Project Structure

├── artifacts/
│   └── solo_metadata.scale        # Metadata exported with subxt
├── src/
│   ├── ec_dilithium_extrinsic.rs               # Sends a signed transfer using sr25519 (Alice's seed)
│   └── generate_dilithium_keypair.rs  # Builds unsigned extrinsic with Dilithium signature
├── alice_seed.txt                 # Hex-encoded Alice seed
├── dilithium_priv.key             # Dilithium private key
├── dilithium_pub.key              # Dilithium public key
├── Cargo.toml
└── README.md

─────────────────────────────────────────────────────
🚀 Features

- ✅ Send transfers signed with sr25519 using `subxt`
- 🔐 Build extrinsics with Dilithium (in progress)
- 🧩 Ready for integration with a custom runtime pallet to verify PQ signatures

─────────────────────────────────────────────────────
🛠️ Requirements

- Rust 1.70+
- `subxt` and `subxt-signer`
- `pqcrypto` (Dilithium)
- Local Solochain node running at `ws://127.0.0.1:9944`
- Export metadata:
  subxt codegen --url ws://127.0.0.1:9944 > artifacts/solo_metadata.scale

─────────────────────────────────────────────────────
⚠️ Important

Provide the cryptographic material:

- `alice_seed.txt` – 32-byte hex seed for sr25519
- `dilithium_priv.key` – Dilithium private key file
- `dilithium_pub.key` – Dilithium public key file

─────────────────────────────────────────────────────
🔁 Usage
# Create the necessary metadata to build extrinsics
subxt metadata -f bytes > artifacts/solo_metadata.scale

# Compile and run EC and Dilithium extrinsic builder
cargo build --bin ec_dilithium_extrinsic --release
cargo run --bin ec_dilithium_extrinsic --release

# Generate Dilithium keypair
cargo build --bin generate_dilithium_keypair --release
cargo run --bin generate_dilithium_keypair --release

```
