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
        suspicious_path
    }
};

static REQ_ID: AtomicU64 = AtomicU64::new(1);

/// Proxy builder for incoming connections
/// Provides atomic req_id started with 1
/// program/script  are optional parameters to call iptables 
pub async fn proxy(
    req: HttpRequest,
    payload: web::Payload,
    upstreams: web::Data<Upstreams>,
    client: web::Data<awc::Client>,
    forbidden_path: web::Data<String>,
    sudo_executor: web::Data<String>,
    script_location: web::Data<String>,
) -> Result<HttpResponse, Error> {

    let req_id = REQ_ID.fetch_add(1, Ordering::Relaxed);
    let peer_addr = req.peer_addr();
    let client_ip = peer_addr.map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string());

    //let path = req.uri().path();
        println!("{:?}", forbidden_path);
        println!("{:?}", sudo_executor);
        println!("{:?}", script_location);
        
    if suspicious_path(forbidden_path.clone(), "/") {

        log::warn!("[req={req_id}] SUSPICIOUS {:?} from {client_ip} — blocking", forbidden_path);
        if let Some(addr) = peer_addr {
            block_ip(
                &sudo_executor.to_string(), 
                &script_location.to_string(), 
                &addr.ip().to_string()
            ).await;
        }
        return Ok(HttpResponse::Forbidden().finish());
    }

    let upstream = upstreams.next();
    let path_and_q = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
    let uri = format!("{}{}", upstream, path_and_q);

    log::info!("[req={req_id}] {client_ip} {} {path_and_q} -> {upstream}", req.method());

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


