use actix_web::HttpResponse;

pub async fn create_user() -> HttpResponse {

    println!("Create user");

    HttpResponse::Ok().finish()
}