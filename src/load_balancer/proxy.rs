use std::sync::atomic::{
    AtomicU64, 
    Ordering
};

use actix_web::{
    web, 
    HttpRequest, 
    HttpResponse, 
    Error
};

use super::{
    models::Upstreams,
    firewall::{
        block_ip,
        is_ip_allowed,
        is_path_allowed,
        matching_forbidden_path,
        normalize_path,
    }
};

use super::models::AllowedIps;
use super::models::ForbiddenPath;
use super::models::ScriptLocation;
use super::models::SudoExecutor;
use super::models::AllowedPath;

static REQ_ID: AtomicU64 = AtomicU64::new(1);

/// Proxy builder for incoming connections
/// Provides atomic req_id started with 1
/// forbidden_path/program/script  are required parameters to call iptables 
pub async fn proxy(
    req: HttpRequest,
    payload: web::Payload,
    upstreams: web::Data<Upstreams>,
    client: web::Data<awc::Client>,
    forbidden_path: web::Data<ForbiddenPath>,
    sudo_executor: web::Data<SudoExecutor>,
    script_location: web::Data<ScriptLocation>,
    allowed_ips: web::Data<AllowedIps>,
    allowed_path: web::Data<AllowedPath> 
) -> Result<HttpResponse, Error> {

    let req_id = REQ_ID.fetch_add(1, Ordering::Relaxed);
    let peer_addr = req.peer_addr();
    let client_ip = peer_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());

    if !is_ip_allowed(&allowed_ips.0, &client_ip) {
        log::warn!("[req={req_id}] {client_ip} not in allowlist — denying");
        return Ok(HttpResponse::Forbidden().finish());
    }

    let sudo_program = sudo_executor.0.clone();
    let blocking_script = script_location.0.clone();

    let upstream = upstreams.next();
    let path_and_q = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
    let uri = format!("{}{}", upstream, path_and_q);

    log::info!("[req={req_id}] {client_ip} {} {path_and_q} -> {upstream}", req.method());

    // Match on the path as the upstream will resolve it, not on the raw
    // request bytes - see normalize_path. The raw path is still what gets
    // forwarded.
    let Some(path_only) = normalize_path(req.uri().path()) else {
        log::warn!("[req={req_id}] malformed path {:?} from {client_ip} — blocking", req.uri().path());
        return deny_and_block(peer_addr.is_some(), sudo_program, blocking_script, &client_ip).await;
    };

    if let Some(p) = matching_forbidden_path(&forbidden_path.0, &path_only) {
        log::warn!("[req={req_id}] SUSPICIOUS {:?} matches forbidden {:?} from {client_ip} — blocking", path_only, p);
        return deny_and_block(peer_addr.is_some(), sudo_program, blocking_script, &client_ip).await;
    }

    if is_path_allowed(&allowed_path.0, &path_only) {
        log::info!("[req={req_id}] allowed path {:?} from {client_ip} — access granted", path_only);
    } else {
        log::warn!("[req={req_id}] not in allowed paths {:?} from {client_ip} — access denied", path_only);
        return deny_and_block(peer_addr.is_some(), sudo_program, blocking_script, &client_ip).await;
    }

    // Pass the upstream body through as-is - otherwise awc decompresses it while
    // the original Content-Encoding/Content-Length headers are still forwarded,
    // and browsers fail with a content encoding error
    let mut fwd = client.request(req.method().clone(), &uri).no_decompress();

    for (name, value) in req.headers() {
        if name == actix_web::http::header::HOST {
            continue;
        }
        fwd = fwd.insert_header((name.clone(), value.clone()));
    }

    let resp = fwd
        .send_stream(payload)
        .await
        .map_err(actix_web::error::ErrorBadGateway)?;

    let status = resp.status();
    log::info!("[req={req_id}] <- {status}");

    let mut builder = HttpResponse::build(status);
    for (name, value) in resp.headers() {
        builder.insert_header((name.clone(), value.clone()));
    }

    Ok(builder.streaming(resp))

}

/// Rejects the request and, when the client address is known,
/// blocks it via the iptables script
async fn deny_and_block(
    peer_known: bool,
    sudo_program: String,
    blocking_script: String,
    client_ip: &str
) -> Result<HttpResponse, Error> {
    if peer_known {
        block_ip(
            sudo_program,
            blocking_script,
            client_ip
        ).await;
    }
    Ok(HttpResponse::Forbidden().finish())
}


