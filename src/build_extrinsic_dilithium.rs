use parity_scale_codec::Encode;
use std::fs;
use pqcrypto_dilithium::dilithium2::{detached_sign, SecretKey};
use pqcrypto_traits::sign::{SecretKey as SecretKeyTrait, DetachedSignature as DetachedSignatureTrait};
use sp_runtime::MultiAddress;
use sp_keyring::AccountKeyring;
use sp_runtime::AccountId32;
use hex;

fn main() {
    // Destinatario: Bob (simulado desde keyring)
    let to: MultiAddress<AccountId32, u32> = MultiAddress::Id(AccountKeyring::Bob.to_account_id());
    let value: u128 = 1_000_000;

    // Codificar el payload
    let payload = (to.clone(), value).encode();

    // Leer la clave secreta de archivo
    let sk_bytes = fs::read("dilithium_priv.key").expect("Clave privada no encontrada");
    let sk = SecretKey::from_bytes(&sk_bytes).expect("Clave privada inválida");

    // Firmar el payload con Dilithium
    let signature = detached_sign(&payload, &sk);

    // Construir el call_data: (pallet_index, function_index, payload, firma)
    let pallet_index = 5u8;
    let function_index = 0u8;
    let call_data = (
        pallet_index,
        function_index,
        payload.clone(),
        signature.as_bytes().to_vec()
    ).encode();

    // Extrínseco sin firmar (0x00 indica unsigned extrinsic)
    let extrinsic = [&[0x00u8][..], &call_data[..]].concat();

    // Mostrar resultado final
    println!("Extrinsic: ", hex::encode(extrinsic));
}
