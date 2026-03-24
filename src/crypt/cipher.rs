use libsodium_rs::{self, ensure_init};
use libsodium_rs::crypto_box;
use std::fs;
use std::path::Path;

/// Encrypt function
/// Server pub key, client secret key and nonce key paths need to be provided 
/// Message is formatted as u8
pub fn cipher(
    server_pub_key: &Path,
    client_secret_key: &Path,
    nonce_secret_key: &Path,
    msg: &[u8]
) -> Result <Vec<u8>, std::io::Error> {
    // Initialize libsodium
    ensure_init().expect("Failed to initialize libsodium");

    // Read Server publickey from files
    let server_pk_bytes: Vec<u8> = fs::read(server_pub_key).unwrap_or_default();

    // Convert Server publickey bytes into CryptoBox
    let spk_box = crypto_box::PublicKey::from_bytes(&server_pk_bytes).unwrap();

    // Read Client secret key from files 
    let client_sk_bytes: Vec<u8> = fs::read(client_secret_key).unwrap_or_default();

    // Convert Client secret key bytes into CryptoBox
    let csk_box = crypto_box::SecretKey::from_bytes(&client_sk_bytes).unwrap();

    // Read nonce data from file and convert it to Nonce struct
    let nonce_from_file= fs::read(nonce_secret_key).unwrap_or_default();
    let s: [u8; 24] = nonce_from_file.try_into().unwrap_or_default();
    let nonce_original = crypto_box::Nonce::from_bytes_exact(s);

    // Encrypt message
    let message = msg;
    let ciphertext = crypto_box::seal(message, &nonce_original, &spk_box, &csk_box).unwrap_or_default();
    
    Ok(ciphertext)

}
