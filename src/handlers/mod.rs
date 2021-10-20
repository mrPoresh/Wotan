mod auth;
mod user;
mod healthy_check;

use healthy_check::healthy_check;
use user::create_user;

use actix_web::web;


pub fn route_config(config: &mut web::ServiceConfig) {

    let healthy_check = web::resource("/healthy").route(web::get().to(healthy_check));

    let create_user = web::resource("/signup").route(web::post().to(create_user));

    config.service(healthy_check).service(create_user);

}