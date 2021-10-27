use actix_web::HttpResponse;
use tracing::{info, instrument};

#[instrument]
pub async fn healthy_check() -> HttpResponse {

    info!("Healthy check works!!!");

    HttpResponse::Ok().finish()
}