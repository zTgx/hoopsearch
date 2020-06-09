pub mod search;

use actix_web::{web};

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(
            web::scope("/search")
            .route("",   web::post().to(search::search))
    );
}



