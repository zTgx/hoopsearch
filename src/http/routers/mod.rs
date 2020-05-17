pub mod indexes;

use actix_web::{web};

pub fn indexes_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v2")
        .service(
            web::scope("/indexes")
            .service(
                // 获取所有的index
                web::resource("").route(web::get().to(indexes::get_indexes))
            )
            .service(
                // 获取index
                web::resource("/{uid}").route(web::get().to(indexes::get_index))
            )
            .service(
                // 创建index
                web::resource("").route(web::post().to(indexes::create_index))
            )
            .service(
                // 更新index
                web::resource("").route(web::put().to(indexes::update_index))
            )
            .service(
                // 删除index
                web::resource("").route(web::delete().to(indexes::delete_index))
            )
        )
    );
}



