use actix_web::{web, App, HttpServer};

use super::{
    client::build_client,
    models::{
        Upstreams,
        ForbiddenPath,
        SudoExecutor,
        ScriptLocation
    },
    proxy::proxy
};

/// Default server config
/// log_level, upstream list and port need to be provided
pub async fn server(
    log_level: String,
    upstream_list: Vec<String>,
    port: u16,
    forbidden_path: Vec<String>,
    sudo_executor: String,
    script_location: String,
) -> std::io::Result<()> {

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let provided_upstreams = upstream_list;
    //let upstreams = Upstreams::new(provided_upstreams);
    let provided_forbidden_paths = forbidden_path;
    log::info!("Listening on 127.0.0.1:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Upstreams::new(provided_upstreams.clone())))
            .app_data(web::Data::new(build_client()))
            .app_data(web::Data::new(ForbiddenPath(provided_forbidden_paths.clone())))
            .app_data(web::Data::new(SudoExecutor(sudo_executor.clone())))
            .app_data(web::Data::new(ScriptLocation(script_location.clone())))
            .default_service(web::route().to(proxy))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await

}
