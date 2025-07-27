use pqcrypto_dilithium::dilithium2::keypair;
use pqcrypto_traits::sign::{PublicKey as PublicKeyTrait, SecretKey as SecretKeyTrait};
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ✅ Generar nuevo par de claves Dilithium
    let (public_key, secret_key) = keypair();

    // ✅ Guardar clave privada en archivo
    let priv_key_bytes = secret_key.as_bytes();
    let mut priv_file = File::create("dilithium_priv2.key")?;
    priv_file.write_all(priv_key_bytes)?;
    println!("🔐 dilithium_priv2.key guardada.");

    // ✅ Guardar clave pública en archivo
    let pub_key_bytes = public_key.as_bytes();
    let mut pub_file = File::create("dilithium_pub2.key")?;
    pub_file.write_all(pub_key_bytes)?;
    println!("🔑 dilithium_pub2.key guardada.");

    Ok(())
}
