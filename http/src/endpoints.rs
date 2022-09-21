use actix_web::web;

mod index;
mod player;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").route(web::get().to(index::handler)))
        .service(web::resource("/player/count").route(web::get().to(player::get_count::handler)))
        .service(
            web::resource("/player/limit/{limit_size}")
                .route(web::get().to(player::gets_by_limit::handler)),
        )
        .service(web::resource("/player/").route(web::post().to(player::bulk_create::handler)))
        .service(web::resource("/player/{id}").route(web::get().to(player::get::handler)))
        .service(web::resource("/player/trade").route(web::put().to(player::trade::handler)));
}
