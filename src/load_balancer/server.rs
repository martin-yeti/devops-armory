use actix_web::{web, App, HttpServer};

use super::{
    client::build_client,
    models::Upstreams,
    proxy::proxy
};

/// Default server config
/// log_level, upstream list and port need to be provided
pub async fn server(
    log_level: String,
    upstream_list: Vec<String>,
    port: u16,
) -> std::io::Result<()> {

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let provided_upstreams = upstream_list;
    let upstreams = Upstreams::new(provided_upstreams);

    log::info!("Listening on 127.0.0.1:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(upstreams.clone()))
            .app_data(web::Data::new(build_client()))
            .default_service(web::route().to(proxy))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await

}
