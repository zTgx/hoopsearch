pub mod routers;
// pub mod error;
pub mod response;

use std::{io, env};
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware};

pub async fn router() -> io::Result<()> {
    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    HttpServer::new(move || 
        App::new()
        .wrap(Cors::new() // allowed_origin return access-control-allow-origin: * by default
        .supports_credentials()
        .finish())
        .wrap(middleware::Logger::default())
        .wrap(middleware::Compress::default())
        .configure(routers::indexes_services)
    )
    .bind(&app_url)?
    .run()
    .await
}