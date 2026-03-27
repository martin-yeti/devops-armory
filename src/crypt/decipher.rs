use libsodium_rs::{self, ensure_init};
use libsodium_rs::crypto_box;
use std::fs;
use std::path::Path;

/// Decrypt function
/// Server secret key, client pub key and nonce key paths need to be provided 
/// Message is formatted as u8
pub fn decipher(
    server_secret_key: &Path,
    client_pub_key: &Path,
    nonce_secret_key: &Path,
    msg: &[u8]
) -> Result<String, std::io::Error> {
    // Initialize libsodium
    ensure_init().expect("Failed to initialize libsodium");

    // Read Server priv key from files
    let server_sk_bytes: Vec<u8> = fs::read(server_secret_key).unwrap();

    // Convert Server priv key bytes into CryptoBox
    let ssk_box = crypto_box::SecretKey::from_bytes(&server_sk_bytes).unwrap();

    // Read Client public key from files 
    let client_pk_bytes: Vec<u8> = fs::read(client_pub_key).unwrap();

    // Convert Client public key bytes into CryptoBox
    let cpk_box = crypto_box::PublicKey::from_bytes(&client_pk_bytes).unwrap();

    let nonce_from_file= fs::read(nonce_secret_key).unwrap();
    let s: [u8; 24] = nonce_from_file.try_into().unwrap();
    let nonce_original = crypto_box::Nonce::from_bytes_exact(s);

    let decrypted = crypto_box::open(&msg, &nonce_original, &cpk_box, &ssk_box).unwrap();
    let x = String::from_utf8(decrypted);
    //assert_eq!(message, &decrypted[..]);
    //println!("{:?}", x.unwrap());

    Ok(x.unwrap())

}
