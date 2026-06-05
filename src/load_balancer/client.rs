use std::sync::Arc;

use rustls::ClientConfig;
use rustls::RootCertStore;

/// Default client builder for HTTPS upstreams
pub fn build_client() -> awc::Client {

    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let tls_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = awc::Connector::new()
        .rustls_0_23(Arc::new(tls_config));

    awc::Client::builder()
        .connector(connector)
        .finish()

}
