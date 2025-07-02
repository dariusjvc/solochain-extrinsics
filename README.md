```
# Solochain Extrinsics Signer – sr25519 + Dilithium

Rust-based project to sign and build extrinsics for a local Solochain node.  
Supports traditional signatures (sr25519 via `subxt`) and post-quantum signatures (Dilithium via `pqcrypto`).

─────────────────────────────────────────────────────
📁 Project Structure

├── artifacts/
│   └── solo_metadata.scale        # Metadata exported with subxt
├── src/
│   ├── ec_signer.rs               # Sends a signed transfer using sr25519 (Alice's seed)
│   └── build_extrinsic_dilithium.rs  # Builds unsigned extrinsic with Dilithium signature
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

# Compile and run sr25519 signer (Alice → Bob)
cargo build --bin ec_signer --release
cargo run --bin ec_signer --release

# Compile and run Dilithium extrinsic builder (unsigned)
cargo build --bin build_extrinsic_dilithium --release
cargo run --bin build_extrinsic_dilithium --release

─────────────────────────────────────────────────────
📌 Roadmap

[x] Send extrinsics signed with sr25519 (Alice)
[ ] Build and send Dilithium-signed extrinsics
[ ] Implement runtime pallet to verify Dilithium signatures
[ ] Support hybrid extrinsics (EC + PQ signature schemes)

```
