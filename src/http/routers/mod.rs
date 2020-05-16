pub mod indexes;

use actix_web::{web};

pub fn indexes_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v2")
        .service(
            web::scope("/indexes")
            .service(
                web::resource("/get/all").route(web::get().to(indexes::get_indexes))
            )
            .service(
                web::resource("/get/{uid}").route(web::get().to(indexes::get_index))
            )
        )
    );
}



