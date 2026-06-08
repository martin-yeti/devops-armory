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
    }
};

use super::models::ForbiddenPath;
use super::models::ScriptLocation;
use super::models::SudoExecutor;

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
) -> Result<HttpResponse, Error> {

    let req_id = REQ_ID.fetch_add(1, Ordering::Relaxed);
    let peer_addr = req.peer_addr();
    let client_ip = peer_addr.map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string());

    let sudo_program = sudo_executor.0.clone();
    let blocking_script = script_location.0.clone();

    let upstream = upstreams.next();
    let path_and_q = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
    let uri = format!("{}{}", upstream, path_and_q);

    log::info!("[req={req_id}] {client_ip} {} {path_and_q} -> {upstream}", req.method());

    let forbidden_path_vec = forbidden_path.0.clone();

    for p in forbidden_path_vec {
        if path_and_q == p {
            log::warn!("[req={req_id}] SUSPICIOUS {:?} from {client_ip} — blocking", p);
            if let Some(addr) = peer_addr {
                block_ip(
                    sudo_program, 
                    blocking_script, 
                    &addr.ip().to_string()
                ).await;
            }
            return Ok(HttpResponse::Forbidden().finish());
        }
    }

    let mut fwd = client.request(req.method().clone(), &uri);

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


