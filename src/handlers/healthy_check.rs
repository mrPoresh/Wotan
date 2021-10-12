use actix_web::HttpResponse;

pub async fn healthy_check() -> HttpResponse {

    println!("Im here");

    HttpResponse::Ok().finish()
}