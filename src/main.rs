mod api;
mod config;
mod job_engine;
mod model;
mod state;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use api::{configure_api, index, json_payload_error, ApiContext};
use config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = AppConfig::from_env().map_err(|error| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, error.to_string())
    })?;

    let bind_addr = config.bind_addr();
    let assets_dir = format!("{}/assets", config.frontend_dir);
    let context = ApiContext::new(config);

    log::info!("starting portfolio server on {bind_addr}");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(context.clone()))
            .app_data(web::JsonConfig::default().error_handler(json_payload_error))
            .configure(configure_api)
            .route("/", web::get().to(index))
            .service(Files::new("/assets", assets_dir.clone()).prefer_utf8(true))
    })
    .bind(bind_addr)?
    .run()
    .await
}
