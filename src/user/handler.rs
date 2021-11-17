use crate::database::Pool;
use crate::user::service as user;
use crate::user::model::{UserData};
use crate::errors::ServiceError;

use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};

// ***              This part of the code is responsible for processing incoming reqests to the server          *** //
//                                                                                                                  //
// * Example ---> fn register                                                                                       //
//                                                                                                                  //
pub async fn register(                                                                                              //
    user_data: web::Json<UserData>,// * this variable is responsible for the incoming JSON Datafrom the request   * //
    pool: web::Data<Pool>,// * this variable is responsible for our database pool                                 * //
) -> Result<HttpResponse, ServiceError> {                                                                           //
// * We pass parameters to the user record function and return the result as a server response and data about user* //
    user::register(user_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))      //
}                                                                                                                   //