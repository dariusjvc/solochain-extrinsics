use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::core::client::ClientT;
use subxt::{OnlineClient, SubstrateConfig};
use subxt_signer::sr25519::{Keypair, SecretKeyBytes};
use pqcrypto_dilithium::dilithium2::{detached_sign, SecretKey as DilithiumSecretKey};
use pqcrypto_traits::sign::SecretKey as SecretKeyTrait;
use std::fs;
use hex;

use pqcrypto_traits::sign::DetachedSignature;

#[subxt::subxt(runtime_metadata_path = "artifacts/solo_metadata.scale")]
pub mod solo {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ===== 1. Preparar mensaje a firmar =====
    let value_to_send: u32 = 42;
    let message = value_to_send.to_le_bytes();

    // ===== 2. Leer clave privada Dilithium y firmar =====
    let sk_bytes = fs::read("dilithium_priv.key")?;
    let dilithium_sk = DilithiumSecretKey::from_bytes(&sk_bytes)
        .map_err(|_| "❌ Error leyendo la clave Dilithium")?;
    let signature = detached_sign(&message, &dilithium_sk);
    let signature_bytes = signature.as_bytes();
    println!("Dilithium signature: 0x{}", hex::encode(signature_bytes));
    let signature_vec = signature.as_bytes().to_vec();

    // ===== 3. Verificar firma vía RPC =====
    let client = HttpClientBuilder::default()
        .build("http://127.0.0.1:9944")?;

    let is_valid: bool = client
        .request("dilithium_verify", (message.to_vec(), signature_vec.clone()))
        .await?;

    if !is_valid {
        println!("❌ Firma Dilithium inválida. No se enviará la extrínseca.");
        return Ok(());
    }

    println!("✅ Firma Dilithium válida. Enviando extrínseca...");

    // ===== 4. Preparar extrínseca normal (sin firma Dilithium) =====
    let hex_seed = fs::read_to_string("alice_seed.txt")?.trim().to_string();
    let seed_bytes: [u8; 32] = hex::decode(hex_seed)?.try_into().map_err(|_| "❌ Seed inválida")?;
    let signer = Keypair::from_secret_key(SecretKeyBytes::from(seed_bytes))?;

    let api = OnlineClient::<SubstrateConfig>::new().await?;
    let tx = solo::tx().template().do_something(value_to_send);

    let events = api.tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    println!("✅ Extrinsic sent successfully. Hash: {:?}", events.extrinsic_hash());

    Ok(())
}
