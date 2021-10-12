mod auth;
mod user;
mod healthy_check;

use healthy_check::healthy_check;

use actix_web::web;


pub fn route_config(config: &mut web::ServiceConfig) {

    let healthy_check = web::resource("/healthy").route(web::get().to(healthy_check));

    config.service(healthy_check);

}