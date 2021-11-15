mod handler;

pub mod model;

pub(crate) mod service;

use crate::user::handler::register;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/user")
            .service(web::resource("/register").route(web::post().to(regiter)))
            //.service -> login
            //.servise -> logout
            //.service -> get me
    );

}