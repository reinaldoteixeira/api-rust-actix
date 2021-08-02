use actix_web::{web, Scope};

use crate::controllers::users;

pub fn get_routers() -> Scope {
    web::scope("")
        .service(users())
}

fn users() -> Scope {
    web::scope("/users")
        .route("", web::get().to(users::list))
        .route("", web::post().to(users::add))
        .route("/{id}", web::get().to(users::detail))
}