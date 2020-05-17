pub mod indexes;

use actix_web::{web};

pub fn indexes_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
            web::scope("/indexes")
            .route("",          web::get().to(indexes::get_indexes))
            .route("/{uid}",    web::get().to(indexes::get_index))
            .route("",          web::post().to(indexes::create_index))
            .route("",          web::put().to(indexes::update_index))
            .route("",          web::put().to(indexes::update_index))
            .route("",          web::delete().to(indexes::delete_index))
    );
}



