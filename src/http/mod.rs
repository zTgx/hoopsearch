pub mod routers;
pub mod error;
pub mod response;
pub mod data;

use std::{io, env};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use crate::http::data::Data;

pub async fn router() -> io::Result<()> {
    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    let db_path = env::var("DB_PATH").expect("DB_PATH not fount.");
    let data = Data::new(db_path);

    HttpServer::new(move || 
        App::new()
        .app_data(web::Data::new(data.clone()))
        .wrap(Cors::new().supports_credentials().finish())
        .wrap(middleware::Logger::default())
        .wrap(middleware::Compress::default())
        .service(
            web::scope("/api/v1").configure(routers::indexes_services)
        )
    )
    .bind(&app_url)?
    .run()
    .await
}