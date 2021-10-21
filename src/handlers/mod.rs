mod auth;
mod user;
mod healthy_check;

use healthy_check::healthy_check;
use user::create_user;

use actix_web::web;
use actix_web::HttpResponse;


pub fn route_config(config: &mut web::ServiceConfig) {

    let healthy_check = web::resource("/healthy").route(web::get().to(healthy_check));

    let auth = web::scope("/auth")
        .service(web::resource("/signup").route(web::post().to(create_user)))
        .service(web::resource("/signin").to(|| HttpResponse::Ok()));

    let user = web::scope("/user")
        .service(web::resource("/userInfo").to(|| HttpResponse::Ok()))
        .service(web::resource("/update").to(|| HttpResponse::Ok()));

    config.service(healthy_check).service(auth).service(user);

}