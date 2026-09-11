use std::fs::File;
use std::io::BufReader;

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, pkcs8_private_keys};

/// Build a rustls server config for TLS termination from a PEM certificate
/// chain and a PEM PKCS#8 private key. Panics on any read/parse failure,
/// since a broken TLS config means the server can't safely start.
pub fn load_rustls_config(cert_path: &str, key_path: &str) -> rustls::ServerConfig {

    let cert_file = File::open(cert_path)
        .unwrap_or_else(|e| panic!("Can't open TLS certificate {cert_path}: {e}"));
    let key_file = File::open(key_path)
        .unwrap_or_else(|e| panic!("Can't open TLS private key {key_path}: {e}"));

    let cert_chain: Vec<CertificateDer<'static>> = certs(&mut BufReader::new(cert_file))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("Can't parse TLS certificate {cert_path}: {e}"));

    let mut keys = pkcs8_private_keys(&mut BufReader::new(key_file))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("Can't parse TLS private key {key_path}: {e}"));

    let key = keys.pop()
        .unwrap_or_else(|| panic!("No PKCS#8 private key found in {key_path}"));

    rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, PrivateKeyDer::Pkcs8(key))
        .unwrap_or_else(|e| panic!("Invalid TLS certificate/key pair: {e}"))

}
