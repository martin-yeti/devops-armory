use libsodium_rs::{self, ensure_init};
use libsodium_rs::crypto_box;

use std::fs;
use std::path::Path;

/// Keystore function
/// Create and store keys as u8 bytes. Keys can be used to crypt/decrypt data
pub fn keystore(
    server_pub_key_path: &Path,
    server_priv_key_path: &Path,
    client_pub_key: &Path,
    client_priv_key: &Path,
    nonce_key: &Path
) {
    // Initialize libsodium
    ensure_init().expect("Failed to initialize libsodium");

    let server_keypair = crypto_box::KeyPair::generate();
    let server_pk = server_keypair.public_key;
    let server_sk = server_keypair.secret_key;
    let client_keypair = crypto_box::KeyPair::generate();
    let client_pk = client_keypair.public_key;
    let client_sk = client_keypair.secret_key;

    // Write Server Keypair into files
    let a = server_pk.as_bytes();
    let b = server_sk.as_bytes();
    fs::write(server_pub_key_path, a).unwrap();
    fs::write(server_priv_key_path, b).unwrap();

    // Write Client Keypair into files
    let c = client_pk.as_bytes();
    let d = client_sk.as_bytes();
    fs::write(client_pub_key, c).unwrap();
    fs::write(client_priv_key, d).unwrap();

    // Generate a random nonce
    let nonce = crypto_box::Nonce::generate();
    fs::write(nonce_key, &nonce).unwrap();

}
