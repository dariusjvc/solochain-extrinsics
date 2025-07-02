#![allow(missing_docs)]

use subxt::{OnlineClient, SubstrateConfig};
use subxt_signer::sr25519::{Keypair, SecretKeyBytes};
use std::fs;

#[subxt::subxt(runtime_metadata_path = "artifacts/solo_metadata.scale")]
pub mod solo {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the seed from file
    let hex_seed = fs::read_to_string("alice_seed.txt")?.trim().to_string();
    let seed_bytes: [u8; 32] = hex::decode(hex_seed)?.try_into().map_err(|_| "Seed must be 32 bytes")?;
    let secret = SecretKeyBytes::from(seed_bytes);
    let keypair = Keypair::from_secret_key(secret)?;
    let signer = keypair; // Keypair implementa el trait Signer automáticamente

    // Conect to the node
    let api = OnlineClient::<SubstrateConfig>::new().await?;

    // Bod Address
    let dest = subxt_signer::sr25519::dev::bob().public_key().into();

    // Crear extrínseco de transferencia
    let tx = solo::tx().balances().transfer_allow_death(dest, 2_000_000);

    // Send the tx and confirm
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    println!("Extrinsic: {:?}", events.extrinsic_hash());

    // if let Some(ev) = events.find_first::<solo::balances::events::Transfer>()? {
    //     println!("Transfer success: {:?}", ev);
    // } else {
    //     println!("Transfer event not found");
    // }

    Ok(())
}
